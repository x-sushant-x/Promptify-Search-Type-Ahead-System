use std::time::SystemTime;

use crate::models::response::suggestions::SuggestionResponse;

use super::trie::Trie;

#[derive(Clone)]
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

    pub fn get_suggestions(&self, prefix: String) -> SuggestionResponse {
        let mut suggestions: SuggestionResponse = SuggestionResponse::default();

        let then = SystemTime::now();

        let suggestions_nodes = self.trie.auto_complete(prefix.as_str());
        
        for node in suggestions_nodes {
            suggestions.suggestions.push(node.word.clone());
            suggestions.total_results = suggestions.total_results + 1;
        }

        let now = SystemTime::now();

        match now.duration_since(then) {
            Ok(time) => {
                suggestions.time_taken = time.as_millis();
            },
            Err(_) => {
                suggestions.time_taken = 0;
            }
        }

        suggestions
    }
}