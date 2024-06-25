use britney::britney::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let b: Britney = Britney::new();
    b.check().await?;
    let code_path = "src/issue.rs";
    let x = b.generate_issue(code_path).await?;
    println!("{:?}", x);
    Ok(())
}
