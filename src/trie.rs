use std::collections::HashMap;

/// Trie structure
pub struct Trie { root: TrieNode, }

/// Node of a Trie structure
pub struct TrieNode {
    key: char,
    children: HashMap<char, TrieNode>,
    is_word: bool,
    is_end: bool
}

impl Trie {
    pub fn new() -> Self {
        Self { root: TrieNode {
            key: '\0',
            children: HashMap::new(),
            is_word: false,
            is_end: false
        }}
    }

    pub fn find(&self, word: &str) -> Option<&TrieNode> {
        let mut ancestor = &self.root;
        for c in word.chars() {
            match ancestor.children.get(&c) {
                Some(lesser_ancestor) => { ancestor = lesser_ancestor },
                None => return None
            }
        }
        Some(ancestor)
    }

    pub fn insert(&mut self, word: &str) -> bool {
        let mut predecessor = &mut self.root;
        let mut end = false;
        for c in word.chars() {
            predecessor = predecessor.children.entry(c).or_insert_with(||
                TrieNode {
                    key: c,
                    children: HashMap::new(),
                    is_word: false, is_end: false
                });
            end = predecessor.is_end;
            predecessor.is_end = false;
        }
        predecessor.is_end = true;
        predecessor.is_word = true;
        !end
    }

    pub fn debug(&self) {
        fn depth_first(node: &TrieNode, prefix: String, is_last: bool) {
            // Don't print anything for the root node (empty key)
            if node.key != '\0' {
                print!("{}", prefix);
                print!("{}", if is_last { "└─" } else { "├─" });
                print!("{}", node.key);
                if node.is_word {
                    print!("]");
                }
                println!();
            }
            
            let children: Vec<_> = node.children.values().collect();
            let child_count = children.len();
            
            for (i, child) in children.iter().enumerate() {
                let is_last_child = i == child_count - 1;
                let new_prefix = if node.key == '\0' {
                    // Root level - no prefix
                    String::new()
                } else {
                    // Add to prefix based on whether parent was last
                    format!("{}{}  ", prefix, if is_last { "" } else { "│" })
                };
                depth_first(child, new_prefix, is_last_child);
            }
        }
        
        depth_first(&self.root, String::new(), true);
    }
}

/// Trait for `Option<&TrieNode>` methods
pub trait OpTN {
    fn is_word(&self) -> bool;
    fn get_all_words(&self) -> Vec<String>;
}

impl OpTN for Option<&TrieNode>{
    fn is_word(&self) -> bool {
        match self {
            Some(n) => n.is_word,
            None => false
        }
    }

    fn get_all_words(&self) -> Vec<String> {
        fn gaw_helper(node: &TrieNode, string_on_branch: &mut Vec<char>, words: &mut Vec<String>) {
            // If this node represents a complete word, save it
            if node.is_word {
                words.push(string_on_branch.iter().collect());
            }
            
            // Explore all children depth-first
            for (ch, child) in &node.children {
                string_on_branch.push(*ch);
                gaw_helper(child, string_on_branch, words);
                string_on_branch.pop();
            }
        }
        let mut ret = Vec::new();
        match self {
            None => ret,
            Some(node) => {
                let mut string_on_branch: Vec<char> = Vec::new();
                gaw_helper(node, &mut string_on_branch, &mut ret);
                ret
            }
        }
    }

}

// Quick Trie Creation macro
#[macro_export]
macro_rules! trie {
    ( $($x: expr),* $(,)? ) => {
        {
            let mut temp_trie = Trie::new();
            $(
                temp_trie.insert($x);
            )*
            temp_trie
        }
    };
}

// Trie Bulk Insertion macro
#[macro_export]
macro_rules! trie_bulk_insert {
    ( $trie:expr, $($x: expr),* $(,)? ) => {
        {
            let mut bulk_ret = true;
            $(
                bulk_ret = bulk_ret && $trie.insert($x);
            )*
            bulk_ret
        }
    };
}
