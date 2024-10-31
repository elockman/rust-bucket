# Winsen ZH10VHT Sensor Library for Rust
A Rust library for reading sensor values from the Winsen ZH10VHT particulate matter sensor. This library allows you to initialize the sensor and read various particulate matter and environmental values.

## Features
-Initialize the particulate matter sensor with the specified serial port and baud rate  
-Read PM1.0, PM2.5, and PM10 concentrations  
-Read temperature and humidity values  

## Installation
Add the following to your Cargo.toml:
```
[dependencies]
serialport = "4.0"
tokio = { version = "1", features = ["full"] }
zh10vht = { path = "../zh10vht" }  # Update the path to where the library is located
```

## Usage
### Example
Here's a basic example of how to use the library in a Rust project:
```
use zh10vht::Zh10vht;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sensor = Zh10vht::init("/dev/ttyS2", 9600).await?;
    let pm1_0 = sensor.read_pm1_0().await?;
    let pm2_5 = sensor.read_pm2_5().await?;
    let pm10 = sensor.read_pm10().await?;
    let temperature = sensor.read_temperature().await?;
    let humidity = sensor.read_humidity().await?;

    println!("PM1.0 concentration: {} µg/m³", pm1_0);
    println!("PM2.5 concentration: {} µg/m³", pm2_5);
    println!("PM10 concentration: {} µg/m³", pm10);
    println!("Temperature: {:.1} °C", temperature);
    println!("Humidity: {:.1} %", humidity);

    Ok(())
}
```

### Functions
```init(port_name: &str, baud_rate: u32) -> io::Result<Self>```: Initializes the sensor with the specified serial port and baud rate.  
```read_pm1_0(&mut self) -> io::Result<f32>```: Reads the PM1.0 concentration from the sensor.  
```read_pm2_5(&mut self) -> io::Result<f32>```: Reads the PM2.5 concentration from the sensor.   ```read_pm10(&mut self) -> io::Result<f32>```: Reads the PM10 concentration from the sensor.  
```read_temperature(&mut self) -> io::Result<f32>```: Reads the temperature value from the sensor.  
```read_humidity(&mut self) -> io::Result<f32>```: Reads the humidity value from the sensor.  
