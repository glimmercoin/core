use std::error::Error;
use glimmer_core::prelude::*;
use glimmer_core::consts::*;
use glimmer_core::block::*;

#[test]
fn validate_changed_chain() {

    
    let mut node = GlimmerNode::new().unwrap();

    let chain = &mut node.chain;
    
    eprintln!("Reserve Bal: {}", chain.get_bal(RESERVE_WALLET));

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    assert!(GlimmerNode::verify_chain(chain));

    chain.chain.get_mut(0).unwrap().txs.get_mut(0).unwrap().amount = 1.0;

    assert_eq!(false, GlimmerNode::verify_chain(chain));


}

#[test]
fn balance() {
    let mut node = GlimmerNode::new().unwrap();

    let chain = &mut node.chain;

    assert_eq!(GENESIS_RESERVE, chain.get_bal(RESERVE_WALLET));

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    assert_eq!(GENESIS_RESERVE - 65.0, chain.get_bal(RESERVE_WALLET));

    assert_eq!(65.0, chain.get_bal("grandma"));

}

// fn print_chain(chain: Vec<Block>) {

//     for block in chain {
//         println!("Block: {}", pretty_hash(&block.hash().to_vec()));
//         println!("Nonce: {}", block.nonce);
//         println!("Parent: {}", pretty_hash(&block.prev_hash.to_vec()));
//         println!("Txs: {}", block.txs.len());
//         for tx in block.txs {
//             println!(" Tx:");
//             println!(" Sender: {}", tx.sender);
//             println!(" Recipient: {}", tx.recipient);
//             println!(" Amount: {}", tx.amount);
//             println!("");
//         }
//         println!("");
//     }
// }

// fn pretty_hash(hash: &Vec<u8>) -> String {
//     let mut string = String::with_capacity(2*hash.len());
//     for n in hash {
//         string.push_str(&format!("{:x}", n));
//     }
//     string
// }
