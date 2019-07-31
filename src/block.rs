use crate::util::*;
use blake2b_rs::blake2b;
use num_bigint::BigUint;
use num_traits::identities::One;
use rayon::prelude::*;

use crate::error::MiningError;
use crate::consts::*;
use crate::tx::Tx;
use crate::util::*;

/// Hash of Blake2b Algorithm
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
            vec.extend(&tx.amount.to_bits().to_be_bytes());
        };

        vec

    }

    /// Returns encoded headers + body of block
    pub fn encode(&self) -> Vec<u8> {
       let mut headers = self.headers();
        // Encode Body
       headers.extend_from_slice(&self.body());

       headers
    }

    /// Returns a hash of the block
    pub fn calculate_hash(encoded: &Vec<u8>, nonce: u64) -> Blake2bHash {
        let mut vec = Vec::with_capacity(encoded.len() + 8);
        // Encode random nonce
        vec.extend_from_slice(encoded);
        vec.extend_from_slice(&convert_u64_to_u8_array(nonce));

        let mut dst = [0; HASH_LEN];

        // Hash it using blake2b
        blake2b(KEY, &vec, &mut dst);

        // Return hash
        dst
    }

    /// Return hash of this block
    pub fn hash(&self) -> Blake2bHash {
        Block::calculate_hash(&self.encode(), self.nonce)
    }

    pub fn try_hash(&self) -> Option<u64> {
      // The target is a number we compare the hash to. It is a 256bit binary with DIFFICULTY
      // leading zeroes.
      let target = get_target();

      let encoded = self.encode();

      // Mine in parallel on CPU
      let valid_nonce = (0..MAX_NONCE).into_par_iter().try_for_each(|nonce| {
          let hash = Block::calculate_hash(&encoded, nonce);
          let hash_int = BigUint::from_bytes_be(&hash);


          if hash_int <= target {
              return Err(nonce)
          }
          Ok(())

      });

      Some(valid_nonce.unwrap_err())
    
      // None
  }


    /// Return the genesis block
    pub fn genesis() -> Self {
        Block::new(vec![Tx::new("-1", RESERVE_WALLET, GENESIS_RESERVE)], [0; HASH_LEN]).unwrap()
        // Block {
        //     timestamp: time().unwrap(),
        //     txs: , RESERVE_WALLET, GENESIS_RESERVE)],
        //     nonce: 100,
        //     prev_hash: [0; HASH_LEN]
        // }
    }
}
