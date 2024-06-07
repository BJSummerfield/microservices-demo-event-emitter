mod graphql;

use graphql::Mutations;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Mutations::create_user().await?;
    Ok(())
}
