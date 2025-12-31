// vibed
mod trie;
mod db;
mod manager;
use std::io::{self, Write};

use crate::manager::TrieManager;

fn main() -> rusqlite::Result<()> {
    println!("🌲 Trie Manager with SQLite");
    println!("============================\n");
    
    // Initialize with database
    let mut manager = TrieManager::new("dictionary.db")?;
    
    println!("Dictionary loaded with {} words\n", manager.db.word_count()?);
    
    // Interactive loop
    loop {
        print!("\nCommands: [a]dd, [s]earch, [c]omplete, [l]ist, [q]uit\n> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        
        match input.chars().next() {
            Some('a') => {
                print!("Enter word to add: ");
                io::stdout().flush().unwrap();
                let mut word = String::new();
                io::stdin().read_line(&mut word).unwrap();
                let word = word.trim();
                
                if manager.add_word(word)? {
                    println!("✓ Added '{}'", word);
                } else {
                    println!("ℹ '{}' already exists", word);
                }
            }
            
            Some('s') => {
                print!("Enter word to search: ");
                io::stdout().flush().unwrap();
                let mut word = String::new();
                io::stdin().read_line(&mut word).unwrap();
                let word = word.trim();
                
                if manager.contains(word) {
                    println!("✓ '{}' exists in dictionary", word);
                } else {
                    println!("✗ '{}' not found", word);
                }
            }
            
            Some('c') => {
                print!("Enter prefix to complete: ");
                io::stdout().flush().unwrap();
                let mut prefix = String::new();
                io::stdin().read_line(&mut prefix).unwrap();
                let prefix = prefix.trim();
                
                let completions = manager.complete(prefix);
                if completions.is_empty() {
                    println!("No completions found");
                } else {
                    println!("Found {} completions:", completions.len());
                    for (i, word) in completions.iter().take(20).enumerate() {
                        println!("  {}. {}", i + 1, word);
                    }
                    if completions.len() > 20 {
                        println!("  ... and {} more", completions.len() - 20);
                    }
                }
            }
            
            Some('l') => {
                manager.trie.debug();
            }
            
            Some('q') => {
                println!("Goodbye!");
                break;
            }
            
            _ => {
                println!("Unknown command");
            }
        }
    }
    
    Ok(())
}
