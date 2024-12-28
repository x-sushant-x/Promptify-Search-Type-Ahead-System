use std::time::Instant;

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
        let now = Instant::now();

        let suggestions_nodes = self.trie.auto_complete(prefix.as_str());

        let mut suggestions: SuggestionResponse = SuggestionResponse::default();
        suggestions.suggestions.reserve(suggestions_nodes.len());

        suggestions.total_results = suggestions_nodes.len();
        
        for node in suggestions_nodes {
            suggestions.suggestions.push(node.word.to_owned());
        }

        suggestions.time_taken = now.elapsed().as_secs_f64() * 1000.0;

        suggestions
    }
}