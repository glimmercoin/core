//! Constant values 
/*
 * Proof of Work
 */

/// Number of leading zeros required for a proof to be valid
pub const POW_DIFFICULTLY: usize = 4;
pub const POW_GOAL: &str = "0000";

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
