mod blockchain; // Import module blockchain

use blockchain::{Blockchain, Transaction}; // Sử dụng struct Blockchain và Transaction từ module blockchain

fn main() {
    let mut blockchain = Blockchain::new(); // Tạo một blockchain mới

    loop {
        // Hiển thị menu cho người dùng chọn hành động
        println!("1. Add a transaction");
        println!("2. Mine a block");
        println!("3. Display the blockchain");
        println!("4. Exit");

        // Đọc lựa chọn của người dùng
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice. Please enter a number.");
                continue;
            }
        };

        // Thực hiện hành động dựa trên lựa chọn của người dùng
        match choice {
            1 => {
                // Nhập giao dịch từ người dùng
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Enter sender name:");
                std::io::stdin().read_line(&mut sender).expect("Failed to read input");
                let sender = sender.trim().to_string();

                println!("Enter receiver name:");
                std::io::stdin().read_line(&mut receiver).expect("Failed to read input");
                let receiver = receiver.trim().to_string();

                println!("Enter amount:");
                std::io::stdin().read_line(&mut amount).expect("Failed to read input");
                let amount: f64 = match amount.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid amount. Please enter a number.");
                        continue;
                    }
                };

                blockchain.add_transaction(Transaction { sender, receiver, amount });
            }
            2 => {
                blockchain.mine_block(); // "Mine" block mới với các giao dịch chờ xử lý
                println!("Block mined successfully!");
            }
            3 => {
                blockchain.display(); // Hiển thị toàn bộ blockchain
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice. Please enter a valid number.");
            }
        }
    }
}
