use britney::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let b: Britney = Britney::new();
    b.check().await?;
    let _ = b
        .answer(
            "Generate 1 good issue with examples based on the code given in \
             my previous message.",
        )
        .await?;
    Ok(())
}
