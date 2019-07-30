//! Glimmer Blockchain utility functions
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::consts::*;
use num_bigint::BigUint;
use num_traits::identities::One;

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

/// This transforms a u64 into a little endian array of u8
pub fn convert_u64_to_u8_array(val: u64) -> [u8; 8] {
    return [
        val as u8,
        (val >> 8) as u8,
        (val >> 16) as u8,
        (val >> 24) as u8,
        (val >> 32) as u8,
        (val >> 40) as u8,
        (val >> 48) as u8,
        (val >> 56) as u8,
    ]
}

pub fn pretty_hash(hash: &Vec<u8>) -> String {
    let mut string = String::with_capacity(2*hash.len());
    for n in hash {
        string.push_str(&format!("{:x}", n));
    }
    string
}

pub fn get_target() -> BigUint {
  BigUint::one() << (HASH_BITS - 4 * POW_DIFFICULTLY)
}
