use britney::{britney::*, issue::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let b: Britney = Britney::new(Some("issue_templates/default.md"));
    b.check().await?;
    let code_path = "src/britney.rs";
    let issue: Issue = b.generate_issues_from_file(code_path, 1).await?;
    b.post_issue(issue).await?;
    Ok(())
}
