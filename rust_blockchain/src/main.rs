// src/main.rs
mod block;
mod blockchain;

use blockchain::Blockchain;
use std::io;

fn main() {
    let mut blockchain = Blockchain::new(4); // 工作量证明的难度

    loop {
        println!("\n选择一个选项:");
        println!("1. 添加区块");
        println!("2. 查看区块链");
        println!("3. 验证区块链");
        println!("4. 退出");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                println!("输入区块的数据:");
                let mut data = String::new();
                io::stdin().read_line(&mut data).unwrap();
                blockchain.add_block(data.trim().to_string());
                println!("区块已添加!");
            }
            "2" => {
                for block in &blockchain.chain {
                    println!("{:?}", block);
                }
            }
            "3" => {
                if blockchain.is_valid() {
                    println!("区块链有效。");
                } else {
                    println!("区块链无效!");
                }
            }
            "4" => {
                println!("退出... 再见，区块链伙伴!");
                break;
            }
            _ => println!("无效选择。请重试。"),
        }
    }
}
