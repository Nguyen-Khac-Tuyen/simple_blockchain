use super::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Digest, Sha256}; // Cần thêm `sha2 = "0.10"` vào Cargo.toml

#[derive(Debug)]
pub struct Block {
    pub index: u32, // Vị trí của block trong chuỗi
    pub timestamp: u128, // Thời gian tạo block
    pub transactions: Vec<Transaction>, // Danh sách giao dịch trong block
    pub previous_hash: String, // Hash của block trước
    pub hash: String, // Hash của block hiện tại
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        // Tạo block mới
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let hash = Block::calculate_hash(index, timestamp, &transactions, &previous_hash);
        Self {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
        }
    }

    pub fn calculate_hash(index: u32, timestamp: u128, transactions: &Vec<Transaction>, previous_hash: &str) -> String {
        // Tính toán hash cho block
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        for transaction in transactions {
            hasher.update(format!("{:?}", transaction)); // Serialize các giao dịch
        }
        hasher.update(previous_hash);
        format!("{:x}", hasher.finalize())
    }
}
