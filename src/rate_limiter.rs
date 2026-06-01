// VecDeque is a double ended queue - we push to the back and pop from the front
use std::collections::VecDeque;

// Everything the rate limiter needs - just three fields
struct RateLimiter {
    // The max number of requests allowed within any single window
    max_requests: usize,
    // The width of the sliding window in seconds
    window_size: u64,
    // a queue of timestamps for recent allowed requests - oldest at front, newest at back
    requests: VecDeque<u64>,
}

// Open the implementation block - all methods live here
impl RateLimiter {

    // Constructor - takes the two limits and returns a fresh empty limiter
    fn new(max_requests: usize, window_size: u64) -> Self {
        // Build and return the struct
        RateLimiter {
            // Field init shorthand: parameter name matches field name, so no ": value" needed
            max_requests,
            // Same shorthand for window size
            window_size,
            // VecDeque::new() gives us an empty queue - no requests recorded yet
            requests: VecDeque::new(),
        }
    }

    // Takes &mut self because everyt call potentially modifies the queue
    fn allow_request(&mut self, timestamp: u64) -> bool {
        // Evict expired entries from the front - keep going until the front is valid or queue is empty
        while let Some(&front) = self.requests.front() {
            // front + window_size <= timestamp means this entry has fallen outsite the window
            if front + self.window_size <= timestamp {
                // Pop and discard the expired entry from the front of the queue
                self.requests.pop_front();
            } else {
                // This entry is still valid - everything behind it is newer so stop here
                break;
            }
        }
        // Check if there is room for one more request in the current window
        if self.requests.len() < self.max_requests {
            // Room available - record this timestamp at the back and allow the request
            self.requests.push_back(timestamp);
            // Return true to signal the caller this request is permitted
            true
        } else {
            // No room - deny without recording anything
            false
        }
    }
}

#[cfg(test)]
mod tests {
    // Pull RateLimiter into scope from the parent file
    use super::*;

    #[test]
    fn test_basic_allow_and_deny() {
        // Build a limiter: at most 3 requests per 10 second window
        let mut rl = RateLimiter::new(3, 10);
        // First request - queue: [1], 1 < 3 -> allowed
        assert_eq!(rl.allow_request(1), true);
        // Second request - queue: [1, 2], 2 < 3 -> allowed
        assert_eq!(rl.allow_request(2), true);
        // Third request - queue: [1, 2, 3], 3 = 3 -> allowed (just at limit)
        assert_eq!(rl.allow_request(3), true);
        // Fourth request - nothing evicted, still 3 in window -> denied
        assert_eq!(rl.allow_request(4), false);
        // ts=12: 1+10 <= 12 evict, 2+10=12 <= 12 evict, queue:[3, 12], 2<3 -> allowed
        assert_eq!(rl.allow_request(12), true);
    }

    #[test]
    fn test_window_boundary() {
        // Limiter: 2 requests per 10-second window
        let mut rl = RateLimiter::new(2, 10);
        // queue: [1]
        assert_eq!(rl.allow_request(1),  true);
        // queue: [1, 10] — at limit
        assert_eq!(rl.allow_request(10), true);
        // ts=11: 1+10=11≤11 → evict ts=1, queue:[10,11], 2<2? no wait — len becomes 1 then 2
        assert_eq!(rl.allow_request(11), true);
        // queue: [10, 11], len=2, not < 2 → denied
        assert_eq!(rl.allow_request(11), false);
    }

    #[test]
    fn test_capacity_one() {
        // Edge case: only 1 request allowed per 5-second window
        let mut rl = RateLimiter::new(1, 5);
        // queue: [1]
        assert_eq!(rl.allow_request(1), true);
        // queue: [1], len=1, not < 1 → denied
        assert_eq!(rl.allow_request(2), false);
        // ts=6: 1+5=6≤6 → evict ts=1, queue:[], push 6 → allowed
        assert_eq!(rl.allow_request(6), true);
        // queue: [6], 6+5=11>6, not evicted, len=1, not < 1 → denied
        assert_eq!(rl.allow_request(6), false);
    }
}