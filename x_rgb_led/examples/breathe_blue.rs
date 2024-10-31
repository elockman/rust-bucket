use x_rgb_led::XRgbLed;

#[tokio::main]
async fn main() {
    let rgb_led = XRgbLed::init(137, 142, 134); // Initialize RGB LED on pins 137, 142, and 134
    rgb_led.config(0, 0, 255, 0, 0, 0, 1).await; // Breathe blue 1Hz
}
