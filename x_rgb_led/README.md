# RGB LED Control Library for Rust
A Rust library for controlling an RGB LED using GPIO pins. This library allows you to initialize the RGB LED, set its color, and create effects like blinking and fading.

## Features
-Initialize the RGB LED with specified GPIO pins  
-Set the RGB LED color with 0-255 values for red, green, and blue  
-Control the blink pattern with customizable on-time, off-time, and number of blinks  
-Configure a fading effect with adjustable fade frequency  

## Installation
Add the following to your Cargo.toml:
```
[dependencies]
tokio = { version = "1", features = ["full"] }
sysfs_gpio = "0.6"
x_rgb_led = { path = "../x_rgb_led" }  # Update the path to where the library is located
```

## Usage
### Example
Here's a basic example of how to use the x_rgb_led library in a Rust project:
```
use x_rgb_led::RgbLed;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let rgb_led = RgbLed::init(17, 27, 22); // Initialize RGB LED on pins 17, 27, and 22
    rgb_led.config(255, 0, 0, 500, 500, 5, 0).await; // Blink red color with 500ms on, 500ms off, for 5 blinks
}
```

### Functions
```init(r_pin: u64, g_pin: u64, b_pin: u64)``` Sets the GPIO pins for the RGB LED.   
```set_color(&self, r: u8, g: u8, b: u8)``` Sets the intensity of the LEDs from 0-255.  
```config(&self, r_max: u8, g_max: u8, b_max: u8, time_on: u64, time_off: u64, blinks: u64, fade_freq: u64)``` Sets the color, blink pattern, and fade effect for the RGB LED.  
```blink(&self, r_max: u8, g_max: u8, b_max: u8, time_on: u64, time_off: u64, blinks: u64)``` Sets the color and blink pattern for the RGB LED.  
```fade(&self, r_max: u8, g_max: u8, b_max: u8, fade_freq: u64)``` Sets the color and fade pattern of the RGB LED.