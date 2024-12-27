use super::trie::Trie;

pub struct TrieQuerySVC {
    trie: Trie
}

impl TrieQuerySVC {
    pub fn new() -> TrieQuerySVC {
        TrieQuerySVC {
            trie: Trie::default()
        }
    }

    pub fn add_search_query(&mut self, query: String) {
        self.trie.insert(query.as_str());
    }

    pub fn get_suggestions(&self, prefix: String) -> Vec<String> {
        let mut suggestions: Vec<String> = Vec::new();
        let suggestions_nodes = self.trie.auto_complete(prefix.as_str());

        
        for node in suggestions_nodes {
            suggestions.push(node.word.clone());
        }

        suggestions
    }
}