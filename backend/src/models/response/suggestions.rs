use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct SuggestionResponse {
    pub time_taken: f64,
    pub total_results: usize,
    pub suggestions: Vec<String>
}