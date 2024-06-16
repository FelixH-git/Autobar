# Autobar
## Dependencies
- `pnpm`
- `cargo`

## Build steps for running
```sh
pnpm install
cargo run
```

## When developing
### Dependencies
- `cargo-watch`

### Steps
```sh
pnpm dlx tailwindcss -i styles/tailwind.css -o static/main.css --watch & # for updating the styles
cargo watch -x run & # for automatic reloads when backend changes
```
