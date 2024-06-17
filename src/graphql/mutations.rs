use fake::{
    faker::{internet::en::SafeEmail, name::en::FirstName},
    Fake,
};
use rand::Rng;
use serde_json::{json, Value};

use crate::graphql::graphql_client::GraphQLQuery;

pub struct Mutations;

impl GraphQLQuery for Mutations {}

impl Mutations {
    pub async fn create_user() -> Result<Value, Box<dyn std::error::Error>> {
        let email: String = SafeEmail().fake();

        let query = json!({
            "query": "mutation CreateUser($email: String!) { createUser(email: $email) { id } }",
            "variables": {
                "email": email
            }
        });

        let response = Self::query_graphql(query).await?;
        Ok(response)
    }

    pub async fn delete_user(id: String) -> Result<Value, Box<dyn std::error::Error>> {
        let query = json!({
            "query": "mutation DeleteUser($id: ID!) { deleteUser(id: $id) { id } }",
            "variables": {
                "id": id
            }
        });

        let response = Self::query_graphql(query).await?;
        Ok(response)
    }

    pub async fn update_name(id: String) -> Result<(), Box<dyn std::error::Error>> {
        let name: String = FirstName().fake();

        let query = json!({
            "query": "mutation UpdateName($id: ID!, $name: String!) { updateName(id: $id, name: $name) { id } }",
            "variables": {
                "id": id,
                "name": name
            }
        });

        Self::query_graphql(query).await?;
        Ok(())
    }

    pub async fn update_birthday(id: String) -> Result<(), Box<dyn std::error::Error>> {
        let birthday = Self::generate_random_date();

        let query = json!({
            "query": "mutation UpdateBirthday($id: ID!, $birthday: String!) { updateBirthday(id: $id, birthday: $birthday) { id } }",
            "variables": {
                "id": id,
                "birthday": birthday
            }
        });

        Self::query_graphql(query).await?;
        Ok(())
    }

    fn generate_random_date() -> String {
        let mut rng = rand::thread_rng();

        let day = rng.gen_range(1..=25).to_string();
        let month = rng.gen_range(1..=12).to_string();
        let year = rng.gen_range(1930..=2024).to_string();

        // Format the date as MM-DD-YYYY
        format!(
            "{:02}-{:02}-{}",
            month.parse::<u32>().unwrap(),
            day.parse::<u32>().unwrap(),
            year
        )
    }
}
