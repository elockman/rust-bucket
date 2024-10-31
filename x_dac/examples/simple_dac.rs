use x_dac::XDac;

#[tokio::main]
async fn main() {
    let dac = XDac::init(17, 90.0, 5000, true); // Example GPIO pin, max duty, fade time, and shunt usage
    dac.set(50.0).await; // Set to 50% output
    dac.fade(50.0, 100.0).await; // Fade from 50% to 100% output
}
