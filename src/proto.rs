///! Glimmer Blockchain UDP Protocool
use std::error::Error;
use std::collections::HashSet;
use crate::chain::Blockchain;
// use uuid::Uuid

// /// UDP Network Request packets
// pub enum ReqPacket {
//     /// Request status of a peer
//     GetStatus,

//     /// Request nodes list
//     GetNodes,

//     /// Regiser Nodes to peer
//     RegisterNodes {
//         /// List of nodes
//         nodes: Vec<String>
//     },

//     /// Request full blockchain
//     GetChain,

//     /// Create Transaction
//     BroadcastTx(Tx)
// }

// /// UDP Network Response Packet
// pub enum ResPacket {
//     Status {
//         /// Name of this mining node
//         // name: Uuid,
//         /// Address Rewards are sent to
//         addr: Option<String>,
//         /// Is this node a miner
//         mining: bool
//     }
//     Ok,
//     /// List Blockchain
//     ChainList {
//         chain: Blockchain,
//         length: usize
//     }, 

//     /// Request nodes list
//     NodesList {
//         nodes: HashSet<String>
//     },

// }
