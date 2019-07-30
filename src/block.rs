use crate::util::*;
use blake2b_rs::blake2b;
use num_bigint::BigUint;
use num_traits::identities::One;

use crate::error::MiningError;
use crate::consts::*;
use crate::tx::Tx;
use crate::util::convert_u64_to_u8_array;

pub type Blake2bHash = [u8; HASH_LEN];

#[derive(Debug, Clone)]
/// Block in the Glimmer blockchain
pub struct Block {
    /// Timestamp of the block creation
    timestamp: u128,
    /// Nonce, or index of block number
    pub nonce: u64,
    /// Hash of previous block
    pub prev_hash: Blake2bHash,

    /// Body of the block
    pub txs: Vec<Tx>
}

impl Block {
    /// Creates a new block
    pub fn new(txs: Vec<Tx>, prev_hash: Blake2bHash) -> Result<Self, MiningError>{
        // Create block
        let mut block = Block {
            timestamp: time().unwrap(),
            txs,
            nonce: 0,
            prev_hash
        };

        block.try_hash()
            .ok_or(MiningError::Iteration)
            .and_then(|nonce| {
                block.nonce = nonce;

                Ok(block)
            })


    }

    /// Append the txs to a block
    pub fn append_tx(&mut self, mut txs: &mut Vec<Tx>) {
        self.txs.append(&mut txs);
    }

    pub fn push_tx(&mut self,  tx: Tx) {
        self.txs.push(tx);
    }

    /// Return a vector of encoded headers
    pub fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend(&convert_u64_to_u8_array(self.timestamp as u64));
        vec.extend_from_slice(&self.prev_hash);

        vec
    }

    pub fn body(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        
        for tx in &self.txs {
            vec.extend_from_slice(tx.sender.as_bytes());
            vec.extend_from_slice(tx.recipient.as_bytes());
            // TODO: Make this actually work
            vec.extend(&convert_u64_to_u8_array(tx.amount as u64));
        };

        vec

    }


    /// Returns a hash of the block
    pub fn calculate_hash(block: &Block, nonce: u64) -> Blake2bHash {
        let mut headers = block.headers();
        headers.extend_from_slice(&block.body());
        headers.extend_from_slice(&convert_u64_to_u8_array(nonce));

        let mut dst = [0; HASH_LEN];

        blake2b(KEY, &headers, &mut dst);

        dst
    }

    /// Return hash of this block
    pub fn hash(&self) -> Blake2bHash {
        Block::calculate_hash(self, self.nonce)
    }

    pub fn try_hash(&self) -> Option<u64> {
      // The target is a number we compare the hash to. It is a 256bit binary with DIFFICULTY
      // leading zeroes.
      let target = BigUint::one() << (HASH_BITS - POW_DIFFICULTLY);

      for nonce in 0..MAX_NONCE {
          let hash = Block::calculate_hash(&self, nonce);
          let hash_int = BigUint::from_bytes_be(&hash);

          if hash_int < target {
              return Some(nonce);
          }
      }

      None
  }


    /// Return the genesis block
    pub fn genesis() -> Self {
        Block {
            timestamp: time().unwrap(),
            txs: vec![Tx::new(RESERVE_WALLET, RESERVE_WALLET, GENESIS_RESERVE)],
            nonce: 100,
            prev_hash: [0; HASH_LEN]
        }
    }
}
