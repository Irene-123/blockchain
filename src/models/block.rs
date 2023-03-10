use super::blockchain::Blockchain; 
use chrono::prelude::*; 
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize}; 

// block a struct that represents a block in a blockchain

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Block {
    pub index: u64, 
    pub timestamp: u64, 
    pub proof_of_work: u64, 
    pub previous_hash: String, 
    pub hash: String
}

impl Block{

    pub fn new(index: u64, previous_hash: String, ) -> Self {
        let block= Block{
            index, 
            timestamp: Utc::now().timestamp_millis() as u64, 
            proof_of_work: u64::default(), 
            previous_hash, 
            hash: String::default(), 
        }; 
        block
    }
        
    // calculate hash 
    pub fn calculate_hash(&self)-> String {
        let mut block_data= self.clone(); 
        block_data.hash= String::default(); 
        let serialized_block_data= serde_json::to_string(&block_data).unwrap();
        let mut hasher= Sha256::new(); 
        hasher.update(serialized_block_data);
        let result= hasher.finalize(); 
        format!("{:x}", result)
        
    }

    //mine block hash 
    pub fn mine(&mut self, blockchain: Blockchain){
        loop {
            if !self.hash.starts_with(&"0".repeat(blockchain.difficulty)){
                self.proof_of_work+=1; 
                self.hash= self.calculate_hash(); 
            }
            else {
                break
            }
        }
    }


}
