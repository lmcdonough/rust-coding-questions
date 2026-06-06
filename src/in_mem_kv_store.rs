// HashMap is all we need - same single import as the log parser
use std::collections::HashMap;

// One field holds the entire store
struct KvStore {
    // Each key maps to its value and an optional expiry timestamp
    // None = lives forever, Some(ts) = expires at absolute timestampt ts
    store: HashMap<String, (String, Option<u64>)>,
}

impl KvStore {

    // No arguments - just an empty store
    fn new() -> Self {
        KvStore {
            store: HashMap::new(),
        }
    }

    // key and value are &str - we own them by calling .to_string() before storing
    fn set(&mut self, key: &str, value: &str, ttl_secs: Option<u64>, now: u64) {
        // Convert Option<ttl_secs> to Option<expiry_timestamp> - None stays None
        let expiry = ttl_secs.map(|ttl| now + ttl);
        // Insert or overwrite - .insert() handles both cases with no extra check needed
        self.store.insert(key.to_string(), (value.to_string(), expiry));
    }

    // &self not &mut - reading only, no writes even when a key is expired
    // Option<&str> borrows from the map - no allocation, no clone needed
    fn get(&self, key: &str, now: u64) -> Option<&str> {
        // Look up the key - ? returns None immediately if the key doesn't exist
        let (val, expiry) = self.store.get(key)?;
        // Check if an expiry was set for this key
        if let Some(exp) = expiry {
            // *exp dereferences the &u64 - now >= expiry means the key has expired
            if now >= *exp {
                // Return None - key exists in the map but is logically deleted
                return None;
            }
        }
        // Key exists and is not expired - return a borrowed &str slice from the stores String
        Some(val.as_str())
    }

    // Removes the key whether it's expired or not - no TTL check needed
    fn delete(&mut self, key: &str) {
        // .remove() returns Option<v> (the old value) - we discard it with a semicolon
        self.store.remove(key);
    }

    fn purge_expired(&mut self, now: u64) {
        self.store.retain(|_, (_, expiry)| {
            expiry.map_or(true, |exp| now < exp)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_set_and_get() {
        let mut kv = KvStore::new();
        // Set a key with no TTL
        kv.set("name", "alice", None, 0);
        // Should returnt the value
        assert_eq!(kv.get("name", 0), Some("alice"));
        // Missing key returns None
        assert_eq!(kv.get("missing", 0), None);
    }

    #[test]
    fn test_ttl_expiry_and_boundary() {
        let mut kv = KvStore::new();
        // Set with TTL=10 at now=0 - expires at timestamp 10
        kv.set("session", "user42", Some(10), 0);
        // Valid before expiry
        assert_eq!(kv.get("session", 5), Some("user42"));
        // Expired exactly at the boundary (now >= expiry)
        assert_eq!(kv.get("session", 10), None);
        // still gone after expiry
        assert_eq!(kv.get("session", 15), None);
    }

    #[test]
    fn test_no_ttl_lives_forever() {
        let mut kv = KvStore::new();
        // None TTL means the key never expires
        kv.set("config", "debug", None, 0);
        // Should still be there far in the future
        assert_eq!(kv.get("config", 9999), Some("debug"));
    }

    #[test]
    fn test_delete_removes_key() {
        let mut kv = KvStore::new();
        kv.set("token", "abc123", None, 0);
        // Delete it
        kv.delete("token");
        // Gone - even though it had no expiry
        assert_eq!(kv.get("token", 0), None);
    }

    #[test]
    fn test_overwrite_resets_ttl() {
        let mut kv = KvStore::new();
        // Set with a short TTL
        kv.set("k", "v1", Some(5), 0);
        // Overwrite with a longer TTL at the same now=0
        kv.set("k", "v2", Some(100), 0);
        // Old TTL of 5 no longer applies - new value and TTL took over
        assert_eq!(kv.get("k", 10), Some("v2"));
    }
}