use std::collections::HashMap;

/// Trie structure
#[derive(Default, Debug)]
pub struct Trie { root: TrieNode, }

/// Node of a Trie structure
#[derive(Default, Debug)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
    is_end: bool
}

impl Trie {
    pub fn new() -> Self {
        Self { root: TrieNode{
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
            predecessor = predecessor.children.entry(c).or_default();
            end = predecessor.is_end;
            predecessor.is_end = false;
        }
        predecessor.is_end = true;
        predecessor.is_word = true;
        !end
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

// TODO: make macros for: trie creation, bulk insertion and debug printer
