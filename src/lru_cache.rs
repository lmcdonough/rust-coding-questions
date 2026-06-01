 // Pull HashMap (0(1) lookup) and BTreeMap (sorted, min = LRU) from std
 use std::collections::{BTreeMap, HashMap};

 // This struct holds everything the cache needs
 struct LRUCache {
    // The hard limit - cache cannot hold more than this many items
    capacity: usize,
    // A monotonically increasing clock - goes up by 1 on every access or insert
    counter: u64,
    // Main store: key maps to (value, clock-tick-when-last-touched)
    map: HashMap<i32, (i32, u64)>,
    // Order tracker: clock-tick maps to key - BTreeMap sorts it so min clock = LRU
    order: BTreeMap<u64, i32>,
 }
 
 // Open the implementation block - all methods for LRUCache go inside here
 impl LRUCache {

    // Constructor - called as LRUCache::new(capacity) - returns a fresh empty cache
    fn new(capacity: i32) -> Self {
        // Build the struct by filling in every field
        LRUCache {
            // Leetcode passes capacity as i32 - cast it to usize since collections use usize
            capacity: capacity as usize,
            // Clock starts at 0 - the first access or insert will bump it to 1
            counter: 0,
            // Empty HashMap - no key/value pairs yet
            map: HashMap::new(),
            // Empty BTreeMap - no order entries yet
            order: BTreeMap::new(),
        }
    }

    // Takes &mut self because a read still updates the order tracking
    fn get (&mut self, key: i32) -> i32 {
        // Copy the value, old_clock) out of the map - .copied() ends the borrow before we mutate
        let entry = self.map.get(&key).copied();
        // Decide what to do based on whether the key exists
        match entry {
            // Key found - val is the stored value, old_clock is when it was last touched
            Some((val, old_clock)) => {
                // Remove the stale clock -> key mapping from the BTreeMap
                self.order.remove(&old_clock);
                // Advance the global clock by 1 - this item is now the most recently used
                self.counter += 1;
                // Overwrite the HashMap entry with the same value but the new clock
                self.map.insert(key, (val, self.counter));
                // Record the new clock -> key mapping in the BTreeMap
                self.order.insert(self.counter, key);
                // Return the value to the counter
                val
            }
            // Key not found - contract says return -1
            None => -1,
        }
    }

    fn put (&mut self, key: i32, value: i32) {
        // Check if the key already exists - copy its old clock out before mutating anything
        let existing = self.map.get(&key).copied();

        if let Some((_, old_clock)) = existing {
            // Key exists: remove its stale lock entry so BTreeMap doesnt keep the old position
            self.order.remove(&old_clock);
        } else if self.map.len() == self.capacity {
            // Key is new AND cache is full - find the LRU item (smallest clock in BTreeMap)
            // .map(|(&clock, &k)|) (clock, k)) copies the keys out so we can mutate right after
            let lru = self.order.iter().next().map(|(&clock, &k)| (clock, k));
            // Unwrap the LRU entry - this always succeeds if capacity > 0
            if let Some((lru_clock, lru_key)) = lru {
                // Evict the LRU items clock entry from the order tracker
                self.order.remove(&lru_clock);
                // Evict the LRU item's value from the main store
                self.map.remove(&lru_key);
            }
        }
        // Advance the clock - this item is now the most recently used
        self.counter += 1;
        // Write the new (value, clock) pair into the HashMap - insert or overwrite
        self.map.insert(key, (value, self.counter));
        // Record clock -> key in BTreeMap so we can find this items order later
        self.order.insert(self.counter, key);
    }
 }

 #[cfg(test)]
 mod tests {
    // Pull everything from the parent file into this test module
    use super::*;

    #[test]
    fn test_basic_lru_eviction() {
        // Build a cache that holds at most 2 items
        let mut cache = LRUCache::new(2);
        // Insert key=1
        cache.put(1, 10);
        // Insert key=2 - cach is now full
        cache.put(2, 20);
        // Read key=1 - this makes key=1 the MRU, key=2 becomes the LRU
        assert_eq!(cache.get(1), 10);
        // Insert key=3 - cache is full, so key=2 (the LRU) gets evicted
        cache.put(3, 30);
        // key=2 was evicted - must return -1
        assert_eq!(cache.get(2), -1);
        // key=3 was just inserted - must return 30
        assert_eq!(cache.get(3), 30);
        // key=1 survived - must return 10
        assert_eq!(cache.get(1), 10);
    }

     #[test]
    fn test_update_existing_key() {
        // Cache with capacity 2
        let mut cache = LRUCache::new(2);
        // Insert two keys
        cache.put(1, 10);
        cache.put(2, 20);
        // Overwrite key=1 with a new value — key=1 becomes MRU, key=2 becomes LRU
        cache.put(1, 999);
        // Insert key=3 — key=2 is LRU so it gets evicted
        cache.put(3, 30);
        // key=1 must be here with the updated value, not the original
        assert_eq!(cache.get(1), 999);
        // key=2 was evicted
        assert_eq!(cache.get(2), -1);
        // key=3 was just inserted
        assert_eq!(cache.get(3), 30);
    }

    #[test]
    fn test_capacity_one() {
        // Edge case: a cache that holds exactly 1 item
        let mut cache = LRUCache::new(1);
        // Insert key=1
        cache.put(1, 10);
        // Insert key=2 — immediately evicts key=1 since capacity=1
        cache.put(2, 20);
        // key=1 is gone
        assert_eq!(cache.get(1), -1);
        // key=2 is the only survivor
        assert_eq!(cache.get(2), 20);
    }   
 }