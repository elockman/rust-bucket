use serialport::prelude::*;
use std::time::Duration;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::serial::SerialStream;

pub struct Zh10vht {
    port: SerialStream,
}

impl Zh10vht {
    pub async fn init(port_name: &str, baud_rate: u32) -> io::Result<Self> {
        let port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open_native_async()
            .await?;

        Ok(Zh10vht { port })
    }

    async fn read_value(&mut self, start_byte: usize) -> io::Result<f32> {
        let mut buffer = [0u8; 32];
        self.port.write_all(&[0x42, 0x4d, 0xe2, 0x01, 0x01, 0xe6]).await?;
        self.port.read_exact(&mut buffer).await?;

        if buffer[0] == 0x42 && buffer[1] == 0x4d {
            let value = ((buffer[start_byte] as u16) << 8 | buffer[start_byte + 1] as u16) as f32 / 10.0;
            Ok(value)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid response from sensor"))
        }
    }

    pub async fn read_pm1_0(&mut self) -> io::Result<f32> {
        self.read_value(2).await
    }

    pub async fn read_pm2_5(&mut self) -> io::Result<f32> {
        self.read_value(6).await
    }

    pub async fn read_pm10(&mut self) -> io::Result<f32> {
        self.read_value(8).await
    }

    pub async fn read_temperature(&mut self) -> io::Result<f32> {
        self.read_value(14).await
    }

    pub async fn read_humidity(&mut self) -> io::Result<f32> {
        self.read_value(16).await
    }
}
