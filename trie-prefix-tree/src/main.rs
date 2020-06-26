use std::collections::HashMap;
struct Trie {
    map: HashMap<char, Trie>,
    endofword: bool,
}

impl Trie {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie {
            map: HashMap::new(),
            endofword: false,
        }
    }

    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut current_node = self;
        for ch in word.chars() {
            if !current_node.map.contains_key(&ch) {
                current_node.map.insert(ch, Trie::new());
            }
            current_node = current_node.map.get_mut(&ch).unwrap();
        }
        current_node.endofword = true;
    }

    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut current_node = self;
        for ch in word.chars() {
            if !current_node.map.contains_key(&ch) {
                return false;
            }
            current_node = current_node.map.get(&ch).unwrap();
        }
        current_node.endofword
    }

    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut current_node = self;
        for ch in prefix.chars() {
            if !current_node.map.contains_key(&ch) {
                return false;
            }
            current_node = current_node.map.get(&ch).unwrap();
        }
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

fn main() {
    println!("Hello, world!");
}
