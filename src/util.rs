//! Glimmer Blockchain utility functions
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

/// Utility function to return the current time in millisseconds
pub fn time() -> Result<u128, Box<dyn Error>> {
    let start = SystemTime::now();
    let timestamp = start.duration_since(UNIX_EPOCH)?.as_millis();
    Ok(timestamp)
}

/// Create a string of length len, filled with char 
pub fn repeat_char(c: char, len: usize) -> String {
    let mut s: String = String::with_capacity(len);
    for _ in 0..len {
        s.push(c)
    }
    s
}
