// File gives us a handle to an open file on disk
use std::fs::File;
// self = the io module (for io::Result), BufRead = read_line() method
// BufReader = the buffered wrapper, Seek + SeekFrom = move the read cursor
use std::io::{self, BufRead, BufReader, Seek, SeekFrom};

// The struct holds everything - position is tracked inside the BufReader automatically
struct TailReader {
    // BufReader<File> wraps the File, adds an internal buffer, and remembers where we are
    reader:  BufReader<File>,
}

impl TailReader {
    // io::Result<Self> is shorthand for Result<TailReader, io::Error>
    fn new(path: &str) -> io::Result<Self> {
        // Try to open the file - ? returns Err immediately if the path doesn't exist
        let file = File::open(path)?;
        // Wrap the File in BufReader - this is what gives us read_line() and buffering
        let mut reader = BufReader::new(file);
        // Seek to the very end - we only want NEW lines, not content already in the file
        reader.seek(SeekFrom::End(0))?;
        // Wrap our struct in Ok() because our return type is Result, not TailReader directly
        Ok(TailReader { reader })
    }

    // &mut self because reading advances the cursor inside the BufReader
    fn read_new_lines(&mut self) -> Vec<String> {
        // Accumulate new lines here before returning them
        let mut lines = Vec::new();
        // Reusable buffer - read_line appends to this, we clear it each iteration
        let mut buf = String::new();
        // read_line returns Ok(bytes_read) - when bytes_read is 0 we've hit EOF
        while self.reader.read_line(&mut buf).unwrap_or(0) > 0 {
            // trim_end strips the trailing \n or \r\n from the line
            lines.push(buf.trim_end().to_string());
            // CRITICAL: clear the buffer or the next read appends to this line
            buf.clear();
        }
        // Return however many new lines we found - empty Vec if nothing new
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Write trait gives us writeln!() on file handles
    use std::io::Write;
    use std::fs;

    #[test]
    fn test_skips_existing_content() {
        let path = "/tmp/tail_test_1.log";
        // Create the file and write two lines that already exist before we open TailReader
        let mut file = fs::File::create(path).unwrap();
        writeln!(file, "old line 1").unwrap();
        writeln!(file, "old line 2").unwrap();
        // Force the write buffer to disk so the file size is correct when we seek
        file.flush().unwrap();
        // TailReader::new seeks to end - old lines should be invisible to us
        let mut tail = TailReader::new(path).unwrap();
        // Nothing new yet - read_new_lines should return empty
        assert_eq!(tail.read_new_lines(), Vec::<String>::new());
        // Open a second handle in append mode - this adds content without truncating
        let mut appender = fs::OpenOptions::new().append(true).open(path).unwrap();
        writeln!(appender, "new line").unwrap();
        // Flush so TailReader can see the new bytes immediately
        appender.flush().unwrap();
        // Now read_new_lines should return exactly the one new line
        assert_eq!(tail.read_new_lines(), vec!["new line"]);
        // Clean up the temp file - .ok() discards the Result so no panic on failure
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_multiple_reads_no_duplication() {
        let path = "/tmp/tail_test_2.log";
        // Start with an empty file
        fs::File::create(path).unwrap();
        let mut tail = TailReader::new(path).unwrap();
        // Open one appender and use it for all writes
        let mut appender = fs::OpenOptions::new().append(true).open(path).unwrap();
        // First batch - write one line, read it back
        writeln!(appender, "line A").unwrap();
        appender.flush().unwrap();
        assert_eq!(tail.read_new_lines(), vec!["line A"]);
        // Second batch - write two more lines
        writeln!(appender, "line B").unwrap();
        writeln!(appender, "line C").unwrap();
        appender.flush().unwrap();
        // Should return B and C only - A must not appear again
        assert_eq!(tail.read_new_lines(), vec!["line B", "line C"]);
        // Third call - nothing new, must return empty
        assert_eq!(tail.read_new_lines(), Vec::<String>::new());
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_nonexistent_file_returns_err() {
        // new() must return Err, not panic, for a missing path
        let result = TailReader::new("/tmp/does_not_exist_xyz.log");
        assert!(result.is_err());
    }
}