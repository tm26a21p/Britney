use britney::{britney::*, issue::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let b: Britney = Britney::new(Some("issue_templates/default.md"));
    b.check().await?;
    let code_path = "src/";
    // let issue: Issue = b.generate_issue_from_file(code_path).await?;
    // println!("{:?}", issue);
    // b.post_issue(issue).await?;
    let issues: Vec<Issue> = b.generate_issues_from_folder(code_path).await?;
    b.post_issues(issues).await?;
    Ok(())
}
