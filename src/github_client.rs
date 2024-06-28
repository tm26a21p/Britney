use dotenv::dotenv;
use octocrab::Octocrab;

use crate::{issue::Issue, issue_template::IssueTemplate};
// use tokio::io::AsyncWriteExt;
// use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct GithubClient
{
    octocrab: Octocrab,
    owner: String,
    repo: String,
}

impl GithubClient
{
    pub fn new() -> Self
    {
        dotenv().ok();
        let owner =
            std::env::var("GITHUB_OWNER").unwrap_or("test".to_string());
        let repo = std::env::var("GITHUB_REPO").unwrap_or("test".to_string());
        let github_token = std::env::var("GITHUB_TOKEN").expect(
            "GITHUB_TOKEN not found. You need one to use the Github API.
            Create a .env file in the root of the project and add \
             GITHUB_TOKEN=your_token_here",
        );
        let octocrab = Octocrab::builder()
            .personal_token(github_token)
            .build()
            .unwrap();
        Self {
            octocrab,
            owner,
            repo,
        }
    }

    pub async fn create_issue(
        &self,
        issue: Issue,
    ) -> octocrab::Result<()>
    {
        let _ = self
            .octocrab
            .issues(&self.owner, &self.repo)
            .create(issue.title.to_owned())
            .body(issue.body)
            .send()
            .await?;

        println!(
            "Issue {} created in Github repository: {}/{}",
            issue.title.to_owned(),
            self.owner,
            self.repo
        );
        Ok(())
    }

    pub async fn create_issues(
        &self,
        issues: Vec<Issue>,
    ) -> octocrab::Result<()>
    {
        for issue in issues {
            self.create_issue(issue).await?;
        }
        Ok(())
    }

    pub async fn create_issues_from_template(
        &self,
        template: IssueTemplate,
        ai_data: Vec<(String, String)>,
    ) -> octocrab::Result<()>
    {
        for (title_data, body_data) in ai_data {
            let issue = template.generate_issue(&title_data, &body_data);
            self.create_issue(issue).await?;
        }
        Ok(())
    }
}
