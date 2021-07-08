//! Constant values 
/*
 * Proof of Work
 */

/// Difficultly level
pub const POW_DIFFICULTLY: usize = 1;


pub const MAX_NONCE: u64 = std::u64::MAX;

/// Length (in bits) of produced hashes
pub const HASH_BITS: usize = 256;
/// Length (in bytes) of pruced hashes
pub const HASH_LEN: usize = HASH_BITS / 8;
/// Key to hash with (empty)
pub const KEY: &[u8] = &[];

/// Wallet address of reserve
pub const RESERVE_WALLET: &str = &"0";
/// Starting number of coins in reserve
pub const GENESIS_RESERVE: f64 = 50_000_000_000_000.0;

// TODO: Make dynamic
/// Mining REWARD
pub const REWARD: f64 = 500.0;

// TODO: make configurable
/// Wallet to send rewards to
pub const MINER_WALLET: &str = "miner";

/*
 * API Settings
 */

// /// PORT to listen on for HTTP API
// pub const API_PORT: u16 = 5000;
// /// ADDR to listen on for HTTP API
// pub const API_ADDR: &str = "0.0.0.0";


/*
 * Glimmer Settings
 */

/// Print debug messages
pub const DEBUG: bool = true;
/// Name of the blockchain
pub const NAME: &str = "Glimmer";
/// IDENT of the blockchain
pub const IDENT: &str = "GLIM";
