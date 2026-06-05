// VecDeque is all we need - same import as the rate limiter
use std::collections::VecDeque;

// Two fields: the window size and the data inside it
struct RollingWindow {
    // How many seconds wide the sliding window is
    window_secs: u64,
    // Each element is a (timestamp, value) pair - oldest at front, newest at back
    data: VecDeque<(u64, f64)>,
}

// Open the implementation block
impl RollingWindow {
    // Takes the window width - no capacity limit needed, window size drives eviction
    fn new(window_secs: u64) -> Self {
        RollingWindow {
            // field init shorthand - parameter name matches the field name exactly
            window_secs,
            // Empty queue - no data points recorded yet
            data: VecDeque::new(),
        }
    }

    // &mut self because we may evict old entries and always push a new one
    fn record(&mut self, timestamp: u64, value: f64) {
        // Evict entries that have fallen outside the sliding window - same pattern as rate limiter
        while let Some(&(ts, _)) = self.data.front() {
            // ts + window_secs <= timestamp means this point is to old to count
            if ts + self.window_secs <= timestamp {
                // Pop the expired entry off the front of the queue
                self.data.pop_front();
            } else {
                // Front is still valid - everything behind it is newer, so stop
                break;
            }
        }
        // Push the new (timestamp, value) pair onto the back of the queue
        self.data.push_back((timestamp, value));
    }

    // &self - reading only, no mutation
    fn average(&self) -> Option<f64> {
        // Return None immediately if nothing is in the window
        if self.data.is_empty() {
            return None;
        }
        // iter() gives &(u64, f64) - destructure to grab just the value, * dereferences &f64 -> f64
        let sum: f64 = self.data.iter().map(|(_, v)| *v).sum();
        // Divide by count - cast len() from usize to f64 so the division works
        Some(sum / self.data.len() as f64)
    }

    fn max(&self) -> Option<f64> {
        // reduce() returns None if empty, Some(max) if not - no is_empty() check needed
        // a.max(b) uses PartialOrd - f64 can't use plain .max() from iterators because of NaN
        self.data.iter().map(|(_, v)| *v).reduce(|a, b| a.max(b))
    }

    fn min(&self) -> Option<f64> {
        // Same shape as max() - just flip to a.min(b)
        self.data.iter().map(|(_, v)| *v).reduce(|a, b| a.min(b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_stats() {
        let mut w = RollingWindow::new(10);
        // Record three data points all within the window
        w.record(1, 100.0);
        w.record(5, 200.0);
        w.record(8, 150.0);
        // (100 + 200 + 150) / 3 = 150.0
        assert_eq!(w.average(), Some(150.0));
        assert_eq!(w.max(), Some(200.0));
        assert_eq!(w.min(), Some(100.0));
    }

    #[test]
    fn test_eviction_changes_stats() {
        let mut w = RollingWindow::new(10);
        w.record(1, 100.0);
        w.record(5, 200.0);
        // ts=12: 1+10=11 so (1, 100.0) evicted - window is now [(5, 200), (12, 300)]
        w.record(12, 300.0);
        // (200 + 300) / 2 = 250.0
        assert_eq!(w.average(), Some(250.0));
        assert_eq!(w.min(), Some(200.0));
        assert_eq!(w.max(), Some(300.0));
    }

    #[test]
    fn test_empty_window_returns_none() {
        // No records at all - all three stat methods must return None
        let w = RollingWindow::new(10);
        assert_eq!(w.average(), None);
        assert_eq!(w.max(), None);
        assert_eq!(w.min(), None);
    }
}

