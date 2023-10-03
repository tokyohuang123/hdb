use std::thread;
use std::time::Duration;

pub struct Timer {
    duration: Duration,
}

impl Timer {
    pub fn new(seconds: u64) -> Self {
        Timer {
            duration: Duration::from_secs(seconds),
        }
    }

    pub fn start<F: Fn() + Send + 'static>(&self, callback: F) {
        let duration = self.duration;
        thread::spawn(move || {
            thread::sleep(duration);
            callback();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer() {
        let timer = Timer::new(2);
        timer.start(|| {
            println!("Timer expired!");
        });
        thread::sleep(Duration::from_secs(3)); // Wait for the timer to expire
    }
}
