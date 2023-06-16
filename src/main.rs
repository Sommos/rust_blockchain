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

impl App {
    fn new() -> Self {
        Self{
            // initialize the blocks vector as empty
            blocks: vec![],
        }
    }

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
}