use reqwest::Client;
use serde_json::Value;
use std::error::Error;

pub trait GraphQLQuery {
    async fn query_graphql(query: Value) -> Result<Value, Box<dyn Error>> {
        let client = Client::new();
        let res = client
            .post("http://localhost:4000/graphql")
            .json(&query)
            .send()
            .await?;

        let response_body: Value = res.json().await?;
        println!("Response: {}", response_body);
        Ok(response_body)
    }
}
