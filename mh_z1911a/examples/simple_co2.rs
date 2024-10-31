use mh_z1911a::MhZ1911a;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut co2_sensor = MhZ1911a::init("/dev/ttyUSB0", 9600).await?;
    let co2 = co2_sensor.read_co2().await?;
    println!("CO2 concentration: {} ppm", co2);

    Ok(())
}
