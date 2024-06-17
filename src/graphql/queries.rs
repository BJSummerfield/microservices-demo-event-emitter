use serde_json::{json, Value};

use crate::graphql::graphql_client::GraphQLQuery;

pub struct Queries;

impl GraphQLQuery for Queries {}

impl Queries {
    pub async fn get_all_users() -> Result<Value, Box<dyn std::error::Error>> {
        let query = json!({
            "query": "query GetAllUsers { getAllUsers { id } }",
        });

        let response = Self::query_graphql(query).await?;
        Ok(response)
    }
}
