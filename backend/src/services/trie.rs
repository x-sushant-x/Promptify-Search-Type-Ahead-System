use std::collections::HashMap;

#[derive(Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    frequency: usize,
    is_end_of_word: bool
}


#[derive(Default)]
pub struct Trie {
    root: TrieNode
}

impl Trie {
    fn insert(&mut self, word: &str, frequency: usize) {
        let mut node = &mut self.root;

        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default)
        }

        node.is_end_of_word = true;
        node.frequency = frequency;
    }

    fn search_prefix(&self, prefix: &str) -> Option<&TrieNode> {
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

    pub fn auto_complete(&self, prefix: &str) -> Vec<String> {
        let mut suggestions = Vec::new();

        if let Some(node) = self.search_prefix(prefix) {
            node.collect_words(prefix.to_string(), &mut suggestions);
        };

        suggestions
    }
}

impl TrieNode {
    fn collect_words(&self, prefix: String, suggestions: &mut Vec<String>) {
        if self.is_end_of_word {
            suggestions.push(prefix.clone());
        }

        for (ch, child) in &self.children {
            child.collect_words(format!("{}{}", prefix, ch), suggestions);
        }
    }
}