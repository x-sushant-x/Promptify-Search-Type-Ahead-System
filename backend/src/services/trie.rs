use std::collections::HashMap;

#[derive(Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool
}


#[derive(Default)]
pub struct Trie {
    root: TrieNode
}

impl Trie {
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default)
        }

        node.is_end_of_word = true
    }

    pub fn search_prefix(&self, prefix: &str) -> Option<&TrieNode> {
        let mut node = &self.root;

        for ch in prefix.chars() {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return None
            }
        }

        Some(node)
    }
}