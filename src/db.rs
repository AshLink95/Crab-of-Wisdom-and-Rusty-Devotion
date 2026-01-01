// prompt engineered
use rusqlite::{Connection, Result};

pub struct TrieDB {
    conn: Connection,
}

pub const DB_PATH: &str = "coward.db";

impl TrieDB {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS words (
                word TEXT PRIMARY KEY,
                added_at INTEGER DEFAULT (strftime('%s', 'now'))
            )",
            [],
        )?;
        
        // Create index for fast prefix searches
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_word_prefix ON words(word)",
            [],
        )?;
        
        Ok(Self { conn })
    }
    
    pub fn insert_word(&self, word: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR IGNORE INTO words (word) VALUES (?1)",
            [word],
        )?;
        Ok(())
    }
    
    pub fn get_all_words(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT word FROM words ORDER BY word DESC")?;
        let words = stmt.query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>, _>>()?;
        Ok(words)
    }
    
    pub fn word_count(&self) -> Result<usize> {
        let count = self.conn.query_row(
            "SELECT COUNT(*) FROM words",
            [],
            |row| row.get::<usize, u32>(0),
        )?;
        Ok(count as usize)
    }
}
