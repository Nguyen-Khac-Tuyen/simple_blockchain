#[derive(Debug, Clone)]
pub struct Transaction {
    pub sender: String, // Người gửi
    pub receiver: String, // Người nhận
    pub amount: f64, // Số tiền giao dịch
}
