mod trie;
use trie::{Trie, OpTN};

fn main() {
    let mut trie = Trie::new();
    
    // Test 1: Basic insertions
    println!("=== Basic Insertions ===");
    assert!(trie.insert("cat"));
    assert!(trie.insert("car"));
    assert!(trie.insert("care"));
    assert!(trie.insert("bombshell"));
    println!("Inserted: cat, car, care");
    
    // Test 2: Find existing words
    println!("\n=== Find Tests ===");
    assert!(trie.find("cat").is_word());
    assert!(trie.find("car").is_word());
    assert!(trie.find("care").is_word());
    assert!(!trie.find("ca").is_word());  // prefix, not word
    println!("Find tests passed");
    
    // Test 3: Prefix clearing (key feature!)
    println!("\n=== Test get_all_words ===");
    let ca_words = trie.find("ca").get_all_words();
    let all_words = trie.find("").get_all_words();
    println!("All words from root: {:?}", all_words);
    println!("All words from ca: {:?}", ca_words);
    
    // Test 4: Duplicates
    println!("\n=== Duplicate Test ===");
    assert!(!trie.insert("cat"));  // Already exists
    println!("Duplicate handling passed");
    
    // Test 5: Edge cases
    println!("\n=== Edge Cases ===");
    let mut empty_trie = Trie::new();
    assert!(empty_trie.insert(""));  // empty word
    assert!(empty_trie.find("").is_some());
    
    assert!(trie.insert(""));  // empty in full trie
    println!("Edge cases passed");
    
    // // Test 6: Print trie (dbg helper)
    // println!("\n=== Trie Debug ===");
    // println!("{:#?}", trie);
    
    println!("\n🎉 All tests passed! Trie works perfectly.");
}
