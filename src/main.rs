extern crate time;
extern crate uuid;
extern crate sha2;
extern crate bincode;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

use sha2::Sha256;
use sha2::Digest;
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Block {
    nonce: Vec<u8>,
    previous_hash: Vec<u8>,
    timestamp: chrono::DateTime<chrono::Utc>,
    data: Vec<u8>,
    hash: Vec<u8>
}

impl Block {
    fn new (data: Vec<u8>, previous_hash: Vec<u8>) -> Block {
        let mut block = Block {
            nonce: Uuid::new_v4().as_bytes().to_vec(),
            previous_hash: previous_hash,
            timestamp: chrono::Utc::now(),
            data: data,
            hash: vec![]
        };

        let mut hasher = Sha256::default();
        let block_data = bincode::serialize(&block, bincode::Infinite).unwrap();
        hasher.input(&block_data);

        block.hash = hasher.result().as_slice().to_vec();
        
        block
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct BlockChain {
    blocks: Vec<Block>
}

impl BlockChain {
    fn new() -> BlockChain {
        BlockChain {
            blocks: vec![]
        }
    }

    fn new_block (&mut self, data: Vec<u8>) {
        let last_block_hash = match self.blocks.last() {
            Some(last) => last.hash.to_owned(), 
            None => vec![0]
        };

        self.blocks.push(Block::new(data, last_block_hash));
    }
}

fn main() {
    let mut blockchain = BlockChain::new();

    blockchain.new_block(vec![0,1]);
    blockchain.new_block(vec![2,3]);
    blockchain.new_block(vec![4,5]);

    println!("{:?}", blockchain);
}
