use crate::repository;

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

    pub async fn get_suggestions(&self, query: String) -> Vec<String> {
        let suggestions = self.repo.get_top_5_suggestions(query).await;

        match suggestions {
            Ok(sugg) => {
                sugg
            }
            Err(err) => {
                println!("Error: {}", err);
                Vec::new()
            }
        }
    }
}
