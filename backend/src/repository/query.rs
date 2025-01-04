use std::sync::Arc;

use sqlx::{prelude::FromRow, Error, PgPool, Row};

pub struct QueryRepo {
    db: Arc<PgPool>,
}

#[derive(Debug, FromRow)]
struct QueryFrequency {
    query: String,
    frequency: i64,
}

impl QueryRepo {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }

    pub async fn add_new_query(&self, query: String) -> Result<(), sqlx::Error> {
        let existing_query = self.get_query(&query).await;

        match existing_query {
            Ok(res) => self.increase_query_frequency(res).await,
            Err(sqlx::Error::RowNotFound) => self.insert_new_query(&query).await,
            Err(err) => Err(err),
        }
    }

    async fn get_query(&self, query: &String) -> Result<QueryFrequency, sqlx::Error> {
        let row: Result<QueryFrequency, sqlx::Error> =
            sqlx::query_as::<_, QueryFrequency>("SELECT * FROM searched_queries WHERE query = $1")
                .bind(&query)
                .fetch_one(&*self.db)
                .await;

        row
    }

    async fn increase_query_frequency(
        &self,
        query_data: QueryFrequency,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE searched_queries SET frequency = $2 WHERE query = $1")
            .bind(query_data.query)
            .bind(query_data.frequency + 1)
            .execute(&*self.db)
            .await?;

        Ok(())
    }

    async fn insert_new_query(&self, query: &String) -> Result<(), Error> {
        sqlx::query("INSERT INTO searched_queries (query, frequency) VALUES ($1, 1)")
            .bind(query)
            .execute(&*self.db)
            .await?;

        Ok(())
    }

    pub async fn get_suggestions(&self, query: String) -> Result<Vec<String>, sqlx::Error> {
        let rows = sqlx::query("SELECT query FROM searched_queries WHERE query LIKE $1 ORDER BY frequency DESC")
        .bind(format!("{}%", query))
        .fetch_all(&*self.db)
        .await?;

        let mut suggestions: Vec<String> = Vec::new();

        for row in rows {
            let suggestion: String = row.get("query");
            suggestions.push(suggestion);
        }

        Ok(suggestions)
    }
}
