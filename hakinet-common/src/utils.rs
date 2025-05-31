use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::time::sleep;

/// Calculate elapsed time in a human-readable format
pub fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;
    let millis = duration.subsec_millis();

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else if seconds > 0 {
        format!("{}.{:03}s", seconds, millis)
    } else {
        format!("{}ms", millis)
    }
}

/// Get current timestamp in seconds since Unix epoch
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Get current timestamp in microseconds since Unix epoch
pub fn current_timestamp_micros() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64
}

/// Format timestamp as human-readable string
pub fn format_timestamp(timestamp: u64) -> String {
    chrono::DateTime::from_timestamp(timestamp as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

/// Measure execution time of an async function
pub async fn measure_time<F, Fut, T>(f: F) -> (T, Duration)
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let start = Instant::now();
    let result = f().await;
    let duration = start.elapsed();
    (result, duration)
}

/// Shuffle a vector using Fisher-Yates algorithm
pub fn shuffle<T>(vec: &mut Vec<T>) {
    use rand::seq::SliceRandom;
    let mut rng = rand::rng();
    vec.shuffle(&mut rng);
}

/// Split a vector into chunks of specified size
pub fn chunks<T: Clone>(vec: Vec<T>, chunk_size: usize) -> Vec<Vec<T>> {
    vec.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect()
}

/// Rate limiter to control operation frequency
pub struct RateLimiter {
    delay: Duration,
    last_call: Option<Instant>,
}

impl RateLimiter {
    pub fn new(delay: Duration) -> Self {
        RateLimiter {
            delay,
            last_call: None,
        }
    }

    pub async fn wait(&mut self) {
        if let Some(last) = self.last_call {
            let elapsed = last.elapsed();
            if elapsed < self.delay {
                sleep(self.delay - elapsed).await;
            }
        }
        self.last_call = Some(Instant::now());
    }
}

/// Convert bytes to human-readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Validate IP address format
pub fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<std::net::IpAddr>().is_ok()
}

/// Validate port number
pub fn is_valid_port(port: u16) -> bool {
    port > 0
}

/// Generate a random delay within a range
pub fn random_delay(min: Duration, max: Duration) -> Duration {
    use rand::Rng;
    let mut rng = rand::rng();
    let min_millis = min.as_millis() as u64;
    let max_millis = max.as_millis() as u64;
    let random_millis = rng.random_range(min_millis..=max_millis);
    Duration::from_millis(random_millis)
}

/// Check if running as root/administrator
pub fn is_privileged() -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::geteuid() == 0 }
    }
    #[cfg(windows)]
    {
        // On Windows, we'll assume we have privileges
        // In a real implementation, you'd check for admin rights
        true
    }
    #[cfg(not(any(unix, windows)))]
    {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
        assert_eq!(format_duration(Duration::from_secs(1)), "1.000s");
        assert_eq!(format_duration(Duration::from_secs(65)), "1m 5s");
        assert_eq!(format_duration(Duration::from_secs(3665)), "1h 1m 5s");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(500), "500 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
    }

    #[test]
    fn test_is_valid_ip() {
        assert!(is_valid_ip("192.168.1.1"));
        assert!(is_valid_ip("::1"));
        assert!(!is_valid_ip("invalid.ip"));
        assert!(!is_valid_ip("256.256.256.256"));
    }

    #[test]
    fn test_is_valid_port() {
        assert!(is_valid_port(80));
        assert!(is_valid_port(65535));
        assert!(!is_valid_port(0));
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(Duration::from_millis(100));
        let start = Instant::now();
        
        limiter.wait().await; // First call should be immediate
        limiter.wait().await; // Second call should wait
        
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
    }
}