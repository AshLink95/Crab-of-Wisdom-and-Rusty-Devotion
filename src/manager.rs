// prompt engineered
use crate::trie::{OpTN, Trie};
use crate::db::TrieDB;

pub struct TrieManager {
    pub trie: Trie,
    pub db: TrieDB,
}

impl TrieManager {
    pub fn new(db_path: &str) -> rusqlite::Result<Self> {
        let db = TrieDB::new(db_path)?;
        let mut trie = Trie::new();
        
        // Load all words from DB into trie
        println!("Loading dictionary from database...");
        for word in db.get_all_words()? {
            trie.insert(&word);
        }
        println!("Loaded {} words", db.word_count()?);
        
        Ok(Self { trie, db })
    }

    // Fast lookup using in-memory trie
    pub fn complete(&self, prefix: &str) -> Vec<String> {
        match self.trie.find(prefix) {
            Some(node) => {
                let suffixes = Some(node).get_all_words();
                suffixes.iter()
                    .map(|suffix| format!("{}{}", prefix, suffix))
                    .collect()
            }
            None => vec![]
        }
    }

    // Add word to both trie and DB
    pub fn add_word(&mut self, word: &str) -> rusqlite::Result<bool> {
        let inserted = self.trie.insert(word);
        self.db.insert_word(word)?;
        Ok(inserted)
    }

    // Check if word exists (fast, uses trie)
    pub fn contains(&self, word: &str) -> bool {
        self.trie.find(word).is_word()
    }
}
