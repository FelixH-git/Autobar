use askama::Template;
use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Form, Router,
};
use derive_more::Display;
use serde::Deserialize;
use serialport::SerialPort;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use strum::{EnumIter, IntoEnumIterator};
use tower_http::services::ServeDir;

async fn show_beverages(State(template): State<ApplicationState>) -> Html<String> {
    Html(template.render().unwrap())
}

async fn order_drink(
    State(ApplicationState { beverages: _, port }): State<ApplicationState>,
    Form(beverage): Form<BeverageParams>,
) -> Html<String> {
    let mut port = port.lock().unwrap();
    let beverage = beverage.beverage;
    let requested_beverage = [beverage.clone() as u8];
    match port.write_all(&requested_beverage) {
        Ok(_) => Html("Du har valt: ".to_owned() + &beverage.to_string()),
        Err(_) => Html("Något gick fel när jag försökte skriva till den seriella porten".into()),
    }
}

#[tokio::main]
async fn main() {
    let port = Arc::new(Mutex::new(
        serialport::new("/dev/ttyS2", 8600)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port"),
    ));

    let root_path = std::env::current_dir().unwrap();
    let app = Router::new()
        .route("/", get(show_beverages))
        .route("/drink", post(order_drink))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/static", root_path.to_str().unwrap())),
        )
        .with_state(ApplicationState {
            beverages: Beverage::iter().map(|b| b.to_string()).collect(),
            port,
        });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template, Clone)]
#[template(path = "beverages.html")]
struct ApplicationState {
    beverages: Vec<String>,
    port: Arc<Mutex<Box<dyn SerialPort>>>,
}

#[derive(Clone, Deserialize)]
struct BeverageParams {
    beverage: Beverage,
}

#[derive(EnumIter, Display, Clone, Deserialize)]
#[repr(u8)]
enum Beverage {
    #[display(fmt = "Gin och Tonic")]
    #[serde(rename = "Gin och Tonic")]
    GinAndTonic = 0x54,
    #[display(fmt = "Rom och Cola")]
    #[serde(rename = "Rom och Cola")]
    RumAndCoke = 0x52,
    #[display(fmt = "Vodka och Cider")]
    #[serde(rename = "Vodka och Cider")]
    VodkaAndCider = 0x56,
}
