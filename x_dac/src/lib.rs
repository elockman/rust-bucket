use tokio::time::{sleep, Duration};
use sysfs_gpio::{Direction, Pin};

pub struct XDac {
    pin: Pin,
    max_duty: f64,
    fade_time: Duration,
    use_shunt: bool,
}

impl XDac {
    pub fn init(pin_number: u64, max_duty: f64, fade_time_ms: u64, use_shunt: bool) -> Self {
        let pin = Pin::new(pin_number);

        pin.export().unwrap();
        pin.set_direction(Direction::Out).unwrap();

        XDac {
            pin,
            max_duty,
            fade_time: Duration::from_millis(fade_time_ms),
            use_shunt,
        }
    }

    pub async fn set(&self, percent: f64) {
        let duty_cycle = (percent / 100.0) * self.max_duty;
        self.set_duty_cycle(duty_cycle).await;
    }

    async fn set_duty_cycle(&self, duty_cycle: f64) {
        let period = Duration::from_millis(1000);
        let on_duration = Duration::from_millis((period.as_millis() as f64 * duty_cycle / 100.0) as u64);
        let off_duration = period - on_duration;

        loop {
            self.pin.set_value(1).unwrap();
            sleep(on_duration).await;
            self.pin.set_value(0).unwrap();
            sleep(off_duration).await;
        }
    }

    pub async fn fade(&self, from_percent: f64, to_percent: f64) {
        let step_time = self.fade_time / ((to_percent - from_percent).abs() as u32);
        let mut current_percent = from_percent;

        while (current_percent - to_percent).abs() > 0.01 {
            self.set(current_percent).await;
            current_percent += if from_percent < to_percent { 1.0 } else { -1.0 };
            sleep(step_time).await;
        }

        self.set(to_percent).await;
    }
}

impl Drop for XDac {
    fn drop(&mut self) {
        self.pin.unexport().unwrap();
    }
}
