use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    frequency: usize,
    pub word: String,
    is_end_of_word: bool
}


#[derive(Default, Clone)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;

        for ch in word.chars() {
            node = node.children.entry(ch.to_ascii_lowercase()).or_insert_with(TrieNode::default)
        }

        node.is_end_of_word = true;
        node.frequency = node.frequency + 1;
        node.word = word.to_string();
    }

    fn search_prefix(&self, prefix: &str) -> Option<&TrieNode> {
        let mut node = &self.root;

        for ch in prefix.chars().map(|c| c.to_ascii_lowercase()) {
            if let Some(next_node) = node.children.get(&ch) {
                node = next_node;
            } else {
                return None
            }
        }

        Some(node)
    }

    pub fn auto_complete(&self, prefix: &str) -> Vec<&TrieNode> {
        let mut suggestions_nodes = Vec::new();

        if let Some(node) = self.search_prefix(prefix) {
            node.collect_words(prefix.to_string(), &mut suggestions_nodes);
        };


        suggestions_nodes.sort_by(|a, b| b.frequency.cmp(&a.frequency));

        suggestions_nodes
    }

    pub fn increase_frequency(&mut self, word: &str, incement_size: usize) {
        let mut node = &mut self.root;

        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(TrieNode::default)
        }

        if node.is_end_of_word {
            node.frequency += incement_size
        }
    }
}

impl TrieNode {
    fn collect_words<'a>(&'a self, prefix: String, suggestions: &mut Vec<&'a TrieNode>) {
        if self.is_end_of_word {
            suggestions.push(self);
        }

        for (ch, child) in &self.children {
            child.collect_words(format!("{}{}", prefix, ch), suggestions);
        }
    }
}