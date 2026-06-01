// HashMap is all we need - one import, one data structure
use std::collections::HashMap;

// The entire parser lives in one struct with one field
struct LogParser {
    // Maps every unique token we've seen to how many times it appeared
    counts: HashMap<String, usize>,
}

// Open the implementation block - all methods go inside here
impl LogParser {
    fn new() -> Self {
        LogParser {
            // Start with an empty map, no tokens counted yet
            counts: HashMap::new(),
        }
    }

    // &mut self because we modify counts, &str because we only need to read the line
    fn add_line(&mut self, line: &str) {
        // split_whitespace() splits on any whitespace and skips empty tokens
        for token in line.split_whitespace() {
            // entry() looks up the key - returns a handle to either slot or a vacant one
            // or_insert(0) fills the vacant slot with 0 if missing, then returns &mut usize either way
            // the * dereferences that &mut usize so += 1 increments the actual value in the map
            *self.counts.entry(token.to_string()).or_insert(0) += 1;
        }
    }

    // &self not &mut self - reading only, no modifications
    fn top_n(&self, n: usize) -> Vec<(String, usize)> {
        // Collect map entries into a Vec so we can sort them - HashMap has no order
        let mut entries: Vec<(String, usize)> = self.counts
            // iter() yields (&String, &usize) pairs - borrowed references
            .iter()
            // clone the key so we own it, copy the count (usize is a Copy so & dereferences it)
            .map(|(k, &v)| (k.clone(), v))
            // consume the iterator into a Vec
            .collect();
        // Sort descending by count - if counts tie, sort ascending by word alphabetically
        entries.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        // Drop everything past position n
        entries.truncate(n);
        entries
    }
}

#[cfg(test)]
mod tests {
    // Bring LogParser into this test module
    use super::*;

    #[test]
    fn test_basic_frequency() {
        // Fresh parser
        let mut p = LogParser::new();
        // Feed it four log lines
        p.add_line("ERROR disk_full");
        p.add_line("ERROR disk_full");
        p.add_line("WARN high_cpu");
        p.add_line("ERROR timeout");
        // top 2 should be ERROR(3) then disk_full(2)
        let result = p.top_n(2);
        assert_eq!(result[0], ("ERROR".to_string(), 3));
        assert_eq!(result[1], ("disk_full".to_string(), 2));
    }

    #[test]
    fn test_tie_broken_alphabetically() {
        // Build a case where two tokens have equal counts
        let mut p = LogParser::new();
        p.add_line("zebra apple zebra apple");
        // Both at count 2 — apple comes first alphabetically
        let result = p.top_n(2);
        assert_eq!(result[0].0, "apple");
        assert_eq!(result[1].0, "zebra");
    }

    #[test]
    fn test_n_larger_than_unique_words() {
        // Ask for top 10 when only 2 unique tokens exist
        let mut p = LogParser::new();
        p.add_line("hello world");
        // Should return both without panicking
        let result = p.top_n(10);
        assert_eq!(result.len(), 2);
    }
}