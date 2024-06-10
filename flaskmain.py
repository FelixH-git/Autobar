from flask import Flask, request, render_template
import serial

app = Flask(__name__)
ser = serial.Serial("/dev/ttyS2")
ser.baudrate = 9600

@app.route('/', methods=['GET', 'POST'])
def beverage():
    selected_beverage = None

    if request.method == "POST":
        selected_beverage = request.form.get('beverage')

    if selected_beverage == "Rum and Coke":
        command = b'\x52'
        ser.write(command)
    
    if selected_beverage == "Gin and Tonic":
        command = b'\x54'
        ser.write(command)
 
    if selected_beverage == "Vodka and Cider":
        command = b'\x56'
        ser.write(command)
 

    return render_template("index.html", selected_beverage=selected_beverage)


if __name__ == "__main__":
    app.run(host="0.0.0.0", debug=True)
