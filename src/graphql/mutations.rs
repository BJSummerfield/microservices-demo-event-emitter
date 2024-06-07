use fake::{
    faker::{internet::en::SafeEmail, name::en::FirstName},
    Fake,
};
use rand::Rng;
use reqwest::Client;
use serde_json::{json, Value};

pub struct Mutations;

impl Mutations {
    async fn query_graphql(query: Value) -> Result<(), Box<dyn std::error::Error>> {
        let client = Client::new();
        let res = client
            .post("http://localhost:4000/graphql")
            .json(&query)
            .send()
            .await?;

        let response_body = res.text().await?;
        println!("Response: {}", response_body);
        Ok(())
    }

    pub async fn create_user() -> Result<(), Box<dyn std::error::Error>> {
        let email: String = SafeEmail().fake();

        let query = json!({
            "query": "mutation CreateUser($email: String!) { createUser(email: $email) { id } }",
            "variables": {
                "email": email
            }
        });

        Self::query_graphql(query).await?;
        Ok(())
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
