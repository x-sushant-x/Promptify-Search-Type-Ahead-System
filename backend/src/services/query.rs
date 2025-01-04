use std::{fs::read_to_string, time::Instant};

use crate::{models::response::suggestions::SuggestionResponse, repository};

pub struct QuerySVC {
    repo: repository::query::QueryRepo,
}

impl QuerySVC {
    pub fn new(repo: repository::query::QueryRepo) -> Self {
        Self { repo }
    }

    pub async fn add_new_query(&self, query: String) -> Result<(), sqlx::Error> {
        self.repo.add_new_query(query).await
    }

    pub async fn get_suggestions(&self, query: String) -> SuggestionResponse {
        let now = Instant::now();

        let suggestions_nodes = self.repo.get_suggestions(query).await;

        let mut suggestions: SuggestionResponse = SuggestionResponse::default();

        let total_suggestions = suggestions_nodes.as_ref().unwrap().len();

        suggestions.suggestions.reserve(total_suggestions);

        suggestions.total_results = total_suggestions;

        match suggestions_nodes {
            Ok(nodes) => {
                for word in nodes {
                    suggestions.suggestions.push(word.to_owned());
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                return suggestions
            }
        }

        suggestions.time_taken = now.elapsed().as_secs_f64() * 1000.0;

        suggestions

        // match suggestions {
        //     Ok(sugg) => {
        //         sugg
        //     }
        //     Err(err) => {
        //         println!("Error: {}", err);
        //         Vec::new()
        //     }
        // }
    }


    pub async fn insert_sample_data_to_db(&self) {
        for line in read_to_string("./datasets/trends.csv").unwrap().lines() {
            let splited = line.split(",");
            let collection = splited.collect::<Vec<&str>>();

            let last = collection[collection.len() - 1];

            self.repo.add_new_query(last.to_string()).await.unwrap();
        }

        for line in read_to_string("./datasets/movies.csv").unwrap().lines() {
            let splited = line.split(",");
            let collection = splited.collect::<Vec<&str>>();

            let last = collection[1];

            self.repo.add_new_query(last.to_string()).await.unwrap();
        }


        for line in read_to_string("./datasets/ecommerce.csv").unwrap().lines() {
            let splited = line.split(",");
            let collection = splited.collect::<Vec<&str>>();

            let last = collection[2];

            self.repo.add_new_query(last.to_string()).await.unwrap();
        }

        for line in read_to_string("./datasets/yahoo.txt").unwrap().lines() {
            self.repo.add_new_query(line.to_string()).await.unwrap();
        }
    }
}
