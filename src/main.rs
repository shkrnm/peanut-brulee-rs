mod user;
mod wallet;

//Do you play DELTARUNE yet? If you don't you should!

use chrono::prelude::*;
use sha2::{Sha256, Digest};
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use wallet::WalletSystem;

#[derive(Debug, Clone)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
}

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: String,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let hash = Block::calculate_hash(index, &timestamp, &transactions, &previous_hash);
        Block { index, timestamp, transactions, previous_hash, hash }
    }

    fn calculate_hash(index: u64, timestamp: &str, transactions: &Vec<Transaction>, previous_hash: &str) -> String {
        let mut hasher = Sha256::new();
        let tx_data: String = transactions.iter()
            .map(|tx| format!("{}{}{}", tx.from, tx.to, tx.amount))
            .collect();
        hasher.update(format!("{}{}{}{}", index, timestamp, tx_data, previous_hash));
        format!("{:x}", hasher.finalize())
    }
}

fn create_genesis_block() -> Block {
    Block::new(0, vec![], "0".to_string())
}

fn add_block(chain: &mut Vec<Block>, transactions: Vec<Transaction>) {
    let previous = chain.last().unwrap();
    let new_block = Block::new(previous.index + 1, transactions, previous.hash.clone());
    chain.push(new_block);
}

fn get_all_addresses(chain: &Vec<Block>) -> HashSet<String> {
    let mut addresses = HashSet::new();
    for block in chain {
        for tx in &block.transactions {
            addresses.insert(tx.from.clone());
            addresses.insert(tx.to.clone());
        }
    }
    addresses
}

fn main() {
    let mut wallet = WalletSystem::new();
    let mut blockchain = vec![create_genesis_block()];
    let mut balances: HashMap<String, u64> = HashMap::new();

    println!("Welcome to peanut-brulee-rs CLI");
    println!("Commands: `new`, `list`, `send <from> <to> <amount>`, `chain`, `known`, `exit`");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["new"] => {
                let addr = wallet.create_account();
                balances.insert(addr.clone(), 0);
                println!("New account created: {}", addr);
            }
            ["list"] => {
                for addr in wallet.list_addresses() {
                    let bal = balances.get(&addr).unwrap_or(&0);
                    println!("{} | Balance: {}", addr, bal);
                }
            }
            ["send", from, to, amount_str] => {
                if let Ok(amount) = amount_str.parse::<u64>() {
                    let sender_balance = balances.get_mut(*from);
                    if let Some(sender) = sender_balance {
                        if *sender >= amount {
                            *sender -= amount;
                            *balances.entry(to.to_string()).or_insert(0) += amount;
                            let tx = Transaction {
                                from: from.to_string(),
                                to: to.to_string(),
                                amount,
                            };
                            add_block(&mut blockchain, vec![tx]);
                            println!("Sent {} from {} to {}", amount, from, to);
                        } else {
                            println!("Insufficient balance.");
                        }
                    } else {
                        println!("Sender not found.");
                    }
                } else {
                    println!("Invalid amount.");
                }
            }
            ["chain"] => {
                for block in &blockchain {
                    println!("{:#?}", block);
                }
            }
            ["known"] => {
                let all = get_all_addresses(&blockchain);
                for addr in all {
                    let bal = balances.get(&addr).unwrap_or(&0);
                    println!("{} | Balance: {}", addr, bal);
                }
            }
            ["exit"] => {
                println!("Goodbye.");
                break;
            }
            _ => {
                println!("Unknown command.");
            }
        }
    }
}
