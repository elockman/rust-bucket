use x_buzzer::XBuzzer;

#[tokio::main]
async fn main() {
    let buzzer = XBuzzer::new(136); // GPIO pin 136 (CHANGE FOR YOUR APPLICATION)
    buzzer.config(1000, 5000, 2000, 3).await; // Frequency: 1000Hz, Alarm Duration: 5000ms, Dwell Duration: 2000ms, Number of Alarms: 3
}
