use britney::britney::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let b: Britney = Britney::new();
    b.check().await?;
    let _ = b.generate_issue();
    Ok(())
}
