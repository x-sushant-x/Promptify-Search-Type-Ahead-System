use std::{fs::read_to_string, time::Instant};

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

    pub fn insert_sample_data(&mut self) {
        for line in read_to_string("./datasets/trends.csv").unwrap().lines() {
            let splited = line.split(",");
            let collection = splited.collect::<Vec<&str>>();

            let last = collection[collection.len() - 1];

            self.trie.insert(last);
        }

        for line in read_to_string("./datasets/movies.csv").unwrap().lines() {
            let splited = line.split(",");
            let collection = splited.collect::<Vec<&str>>();

            let last = collection[1];

            self.trie.insert(last);
        }


        for line in read_to_string("./datasets/ecommerce.csv").unwrap().lines() {
            let splited = line.split(",");
            let collection = splited.collect::<Vec<&str>>();

            let last = collection[2];

            self.trie.insert(last);
        }
    }
}