use std::error::Error;
use glimmer_core::prelude::*;
use glimmer_core::consts::*;
use glimmer_core::block::*;

fn main() -> Result<(), Box<dyn Error>> {
    
    let mut node = GlimmerNode::new()?;

    let chain = &mut node.chain;

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    chain.add_tx(Tx::new(RESERVE_WALLET, "grandma", 65.0));

    chain.add_block();

    print_chain(chain.chain.clone());

    Ok(())
}

fn print_chain(chain: Vec<Block>) {

    for block in chain {
        println!("Block: {}", pretty_hash(&block.hash().to_vec()));
        println!("Nonce: {}", block.nonce);
        println!("Parent: {}", pretty_hash(&block.prev_hash.to_vec()));
        println!("Txs: {}", block.txs.len());
        for tx in block.txs {
            println!(" Tx:");
            println!(" Sender: {}", tx.sender);
            println!(" Recipient: {}", tx.recipient);
            println!(" Amount: {}", tx.amount);
            println!("");
        }
        println!("");
    }
}

fn pretty_hash(hash: &Vec<u8>) -> String {
    let mut string = String::with_capacity(2*hash.len());
    for n in hash {
        string.push_str(&format!("{:x}", n));
    }
    string
}
