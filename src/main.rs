// import necessary dependencies
use crypto_hash::{hex_digest, Algorithm};

pub struct App {
    // specify the type parameter for the Vec
    pub blocks: Vec,
}

#[derive(Serialize, Deserialize, debug,clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

const DIFFICULTY_PREFIX: &str = "00";

// convert a hash to binary
fn hash_to_binary(hash: &[u8]) -> String {
    // create an empty string
    let mut res: String = String::default();
    // iterate over the hash
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    // return the binary string
    res
}

impl App {
    fn new() -> Self {
        Self{
            // initialize the blocks vector as empty
            blocks: vec![],
        }
    }

    // create the genesis block
    fn genesis(&mut self) {
        // save the current timestamp
        let timestamp = UTC::now().timestamp();
        // set the previous hash 
        let previous_hash = String::from("genesis");
        // set the block data
        let data = String::from("genesis!");
        // set the nonce
        let nonce = 2836;
        // create a string of the block data
        let block_string = format!(
            "{}{}{}{}", 
            timestamp,
            previous_hash, 
            data, 
            nonce, 
            self.blocks.len()
        );
        // compute the hash of the block string using SHA-256
        let hash = hex_digest(Algorithm::SHA256, block_string.as_bytes());

        // create the genesis block with the computed values
        let genesis_block = Block{
            id: 0,
            timestamp,
            previous_hash,
            data,
            nonce,
            hash,
        };
        // push the genesis block to the blocks vector
        self.blocks.push(genesis_block);
    }

    // add a new block to the blockchain
    fn add_new_block(&mut self, block: Block) {
        // get the latest block
        let latest_block = self.blocks.last().expect("there is at least one block");
        // check if the block is valid
        if self.is_block_valid(&block, latest_block) {
            // push the block to the blocks vector
            self.blocks.push(block);
        // if the block is not valid, log an error
        } else {
            error!("invalid block");
        }
    }

    // check if the block is valid
    fn is_block_valid(&self, block: &block, previous_block: &block) -> bool {
        if block.previous_hash != previous_block.hash {
            warn!("block with id: {} has wrong previous hash", block.id);
            return false;
        } else if !hash_to_binary(&hex::decode(&block.hash).expect("can decode from hex")).starts_with(DIFFICULTY_PREFIX) {
            warn!("block with id: {} has invalid difficulty", block.id);
            return false;
        } else if block.id != previous_block.id + 1 {
            warn!("block with id: {} is not the next block after previous block with id: {}", block.id, previous_block.id);
        }
    }
}