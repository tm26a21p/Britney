use britney::*;

#[tokio::main]
async fn main() -> octocrab::Result<()>
{
    let client = GithubClient::new();

    let template =
        IssueTemplate::new("Template Title: {title}", "Template Body: {body}");

    // let ai_data = generate_ai_content();

    client
        .create_issues_from_template(template, ai_data)
        .await?;

    Ok(())
}
