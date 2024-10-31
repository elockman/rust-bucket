# Buzzer Library for Rust
A simple Rust library for controlling a piezo buzzer using GPIO pins. This library allows you to initialize the buzzer, and control it with specified frequencies and durations, including support for repetitive alarms.

## Features
-Initialize the buzzer with a specified GPIO pin  
-Control the buzzer frequency and duration  
-Set the number of alarms (with support for indefinite alarms)  
-Easy integration with asynchronous tasks using tokio

## Installation
Add the following to your Cargo.toml:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
sysfs_gpio = "0.6"
buzzer = { path = "../buzzer" }  # Update the path to where the library is located
```
## Usage
### Example
Here's a basic example of how to use the buzzer library in a Rust project:

```
use buzzer::Buzzer;

use tokio::time::Duration;  

#[tokio::main]  
async fn main() {  
    let buzzer = Buzzer::init(17);  // Initialize the buzzer on GPIO pin 17  
    buzzer.config(1000, 5000, 2000, 3).await; // Frequency: 1000Hz, Alarm Duration: 5000ms, Dwell Duration: 2000ms, Number of Alarms: 3
}
```

### Functions
```init(pin_number: u64) -> Self```: Sets up the GPIO pin for the buzzer.
</br></br></br>
```config(&self, frequency: u64, alarm_duration: u64, dwell_duration: u64, num_alarms: u64)```: Controls the buzzer's frequency, alarm duration, dwell duration, and number of alarms.
