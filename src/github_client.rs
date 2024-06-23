use dotenv::dotenv;
use octocrab::Octocrab;

use crate::issue::IssueTemplate;
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

    pub async fn create_issues(
        &self,
        issues: Vec<Issue>,
    ) -> octocrab::Result<()>
    {
        for issue in issues {
            let issue = self
                .octocrab
                .issues(&self.owner, &self.repo)
                .create(issue.title)
                .body(issue.body)
                .send()
                .await?;

            println!("Created issue: {:?}", issue);
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

            let created_issue = self
                .octocrab
                .issues(&self.owner, &self.repo)
                .create(&issue.title)
                .body(&issue.body)
                .send()
                .await?;

            println!("Created issue: {:?}", created_issue);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Issue
{
    pub title: String,
    pub body: String,
}

impl Issue
{
    pub fn new() -> Self
    {
        Self {
            title: String::new(),
            body: String::new(),
        }
    }

    pub fn from_template(_issue_template: IssueTemplate) -> Self
    {
        unimplemented!()
    }
}
