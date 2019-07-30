//! Constant values 
/*
 * Proof of Work
 */

/// Number of leading zeros required for a proof to be valid
pub const POW_DIFFICULTLY: usize = 4;
pub const POW_GOAL: &[u8]= &[0; POW_DIFFICULTLY];

pub const MAX_NONCE: u64 = 1_000_000;

pub const HASH_BITS: usize = 96;
pub const HASH_LEN: usize = HASH_BITS / 8;
pub const KEY: &[u8] = &[0];

pub const RESERVE_WALLET: &str = &"0";
pub const GENESIS_RESERVE: f64 = 50_000_000_000.0;
pub const REWARD: f64 = 500.0;

pub const MINER_WALLET: &str = "avery";

/*
 * API Settings
 */

/// PORT to listen on for HTTP API
pub const API_PORT: u16 = 5000;
/// ADDR to listen on for HTTP API
pub const API_ADDR: &str = "0.0.0.0";


/*
 * Glimmer Settings
 */

/// Print debug messages
pub const DEBUG: bool = true;
/// Name of the blockchain
pub const NAME: &str = "Glimmer";
/// IDENT of the blockchain
pub const IDENT: &str = "GLIM";
