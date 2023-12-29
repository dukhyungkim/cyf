use std::time::Duration;
use tokio::time;

pub struct RateLimiter {
    last_request: Option<time::Instant>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            last_request: None
        }
    }

    pub fn is_allowed(&mut self) -> bool {
        match self.last_request {
            Some(prev) => {
                let now = time::Instant::now();
                let duration = now.duration_since(prev);
                if duration < Duration::from_secs(1) {
                    false
                } else {
                    self.last_request = Some(now);
                    true
                }
            }
            None => {
                self.last_request = Some(time::Instant::now());
                true
            }
        }
    }
}