pub mod block;
pub mod transaction; // Import các module con

pub use block::Block; // Export struct Block
pub use transaction::Transaction; // Export struct Transaction

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>, // Chuỗi các block
    pending_transactions: Vec<Transaction>, // Danh sách giao dịch đang chờ xử lý
}

impl Blockchain {
    pub fn new() -> Self {
        // Tạo block khởi đầu (Genesis block)
        let genesis_block = Block::new(0, vec![], String::from("0"));
        Self {
            chain: vec![genesis_block],
            pending_transactions: vec![],
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        // Thêm giao dịch mới vào danh sách chờ xử lý
        self.pending_transactions.push(transaction);
    }

    pub fn mine_block(&mut self) {
        // "Mine" block mới
        let previous_block = self.chain.last().expect("Blockchain should have at least one block");
        let new_block = Block::new(
            self.chain.len() as u32,
            self.pending_transactions.clone(),
            previous_block.hash.clone(),
        );
        self.chain.push(new_block);
        self.pending_transactions.clear(); // Làm sạch danh sách giao dịch chờ
    }

    pub fn display(&self) {
        // Hiển thị toàn bộ blockchain
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}
