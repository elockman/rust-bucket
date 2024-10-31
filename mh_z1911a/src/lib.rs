use serialport::prelude::*;
use std::time::Duration;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::serial::SerialStream;

pub struct MhZ1911a{
    port: SerialStream,
}

impl MhZ1911a {
    pub async fn init(port_name: &str, baud_rate: u32) -> io::Result<Self> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open_native_async()
            .await?;

        Ok(MhZ1911a { port })
    }

    pub async fn read_co2(&mut self) -> io::Result<u16> {
        let mut buffer = [0u8; 9];
        self.port.write_all(&[0xff, 0x01, 0x86, 0, 0, 0, 0, 0, 0x79]).await?;
        self.port.read_exact(&mut buffer).await?;

        if buffer[0] == 0xff && buffer[1] == 0x86 {
            let co2 = ((buffer[2] as u16) << 8) | buffer[3] as u16;
            Ok(co2)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid response from sensor"))
        }
    }
}
