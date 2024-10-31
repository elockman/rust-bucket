# Winsen MH-Z1911A CO2 Sensor Library for Rust
A Rust library for interfacing with the Winsen MH-Z1911A CO2 sensor over a serial connection. This library allows you to initialize the sensor and read CO2 concentration values.

## Features
-Initialize the CO2 sensor with the specified serial port and baud rate  
-Read CO2 concentration values from the sensor  

## Installation
Add the following to your Cargo.toml:
```
[dependencies]
serialport = "4.0"
tokio = { version = "1", features = ["full"] }
mh_z1911a = { path = "../mh_z1911a" }  # Update the path to where the library is located
```

## Usage
### Example
Here's a basic example of how to use the x_dac library in a Rust project:
```
use mh_z1911a::MhZ1911a;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sensor = MhZ1911a::init("/dev/ttyS3", 9600).await?;
    let co2 = sensor.read_co2().await?;
    println!("CO2 concentration: {} ppm", co2);

    Ok(())
}
```

### Functions
```init(port_name: &str, baud_rate: u32) -> io::Result<Self>```: Initializes the sensor with the specified serial port and baud rate.  
```read_co2(&mut self) -> io::Result<u16>```: Reads the CO2 concentration from the sensor.  
