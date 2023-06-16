pub struct App {
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