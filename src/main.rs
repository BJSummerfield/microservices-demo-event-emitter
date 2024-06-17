mod emitter;
mod graphql;

use emitter::Emitter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut emitter = Emitter::new(50, 10);
    emitter.start().await;
    Ok(())
}
