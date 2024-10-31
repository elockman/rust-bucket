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
