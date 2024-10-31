# DAC Library for Rust
A Rust library for controlling a 0-10V (or 4-20mA) output using a PWM duty cycle. This library allows you to initialize the DAC, set the output voltage, and configure fading transitions.

## Features
-Initialize the DAC with a specified GPIO pin  
-Control the output voltage from 0 to 10V  
-Set the number of blinks (with support for indefinite blinks)  
-Configure fading transitions between voltage levels  
-Support for 0-10V output using an optional shunt  

## Installation
Add the following to your Cargo.toml:
```
[dependencies]
tokio = { version = "1", features = ["full"] }
sysfs_gpio = "0.6"
x_dac = { path = "../x_dac" }  # Update the path to where the library is located
```

## Usage
### Example
Here's a basic example of how to use the x_dac library in a Rust project:
```
use x_dac::XDac;

#[tokio::main]
async fn main() {
    let dac = XDac::init(17, 90.0, 5000, true); // Initialize DAC on GPIO pin 17, with max duty 90%, fade time 5000ms, and shunt enabled
    dac.set(50.0).await; // Set the output to 50% (5V)
    dac.fade(0.0, 100.0).await; // Fade from 0% (0V) to 100% (10V)
}
```

### Functions
```init(pin_number: u64, max_duty: f64, fade_time_ms: u64, use_shunt: bool) -> Self```: Sets up the GPIO pin and configuration for the DAC.   

```set(&self, percent: f64) -> impl Future```: Sets the desired output level in percent from 0-100.   

```fade(&self, from_percent: f64, to_percent: f64) -> impl Future```: Fades the output voltage from one percentage level to another over the specified fade time.   
