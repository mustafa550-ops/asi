use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct CloudRateLimiter {
    requests: VecDeque<Instant>,
    max_requests: u32,
    window: Duration,
    retry_count: u32,
}

impl CloudRateLimiter {
    pub fn new(max_requests: u32, window_secs: u64) -> Self {
        Self {
            requests: VecDeque::new(),
            max_requests,
            window: Duration::from_secs(window_secs),
            retry_count: 0,
        }
    }

    pub fn check(&mut self) -> Result<(), String> {
        let now = Instant::now();
        while let Some(&t) = self.requests.front() {
            if now.duration_since(t) > self.window {
                self.requests.pop_front();
            } else {
                break;
            }
        }

        if self.requests.len() >= self.max_requests as usize {
            self.retry_count += 1;
            let wait = self.window.as_secs() as f64 * 2_f64.powi(self.retry_count.min(5) as i32);
            return Err(format!("Rate limit asildi. {}ms sonra tekrar deneyin.", (wait * 1000.0) as u64));
        }

        self.requests.push_back(now);
        self.retry_count = 0;
        Ok(())
    }

    pub fn remaining(&self) -> u32 {
        self.max_requests.saturating_sub(self.requests.len() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_ok() {
        let mut limiter = CloudRateLimiter::new(10, 60);
        assert!(limiter.check().is_ok());
    }

    #[test]
    fn test_rate_limit_exceeded() {
        let mut limiter = CloudRateLimiter::new(2, 60);
        assert!(limiter.check().is_ok());
        assert!(limiter.check().is_ok());
        assert!(limiter.check().is_err());
    }

    #[test]
    fn test_remaining() {
        let mut limiter = CloudRateLimiter::new(5, 60);
        assert_eq!(limiter.remaining(), 5);
        let _ = limiter.check();
        assert_eq!(limiter.remaining(), 4);
    }
}
