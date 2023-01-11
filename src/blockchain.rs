use chrono::prelude::*; 
use super::block::Block; 

use super::block::*; 
type Blocks= Vec<Block>; 

// Blockchain a struct that represents the blockchain

#[derive(Debug)]
pub struct Blockchain{
    pub genesis_block: Block, 
    pub chain: Blocks, 
    pub difficulty: usize
}

impl Blockchain{
    pub fn new(difficulty: usize) -> Self {
        let mut genesis_block= Block{
            index=0, 
            timestamp: Utc::now().timestamp_millis() as u64,
            proof_of_work: u64::default(), 
            previous_hash: String::default(),
            hash: String::default()
        };
        let chain= Vec::new(); 
        chain.push(genesis_block.clone()); 
        //create a blockchain instance 
        let blockchain= Blockchain{
            genesis_block, 
            chain, 
            difficulty
        };
        blockchain

    }
    pub fn add_block(&mut self, nonce: String) {
        let new_block= Block::new(
            self.chain.len() as u64, 
            nonce, 
            self.chain[&self.chain.len()-1].previous_hash.clone()
        ); 
        new_block.mine(self.clone()); 
        self.chain.push(new_block.clone()); 
        println!("New block added to chain -> {:?}", new_block); 
    }
}

