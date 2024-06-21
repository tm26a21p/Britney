use dotenv::dotenv;
use octocrab::Octocrab;

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
            std::env::var("GITHUB_OWNER").expect("GITHUB_OWNER not found.");
        let repo =
            std::env::var("GITHUB_REPO").expect("GITHUB_REPO not found.");
        let github_token =
            std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found.");
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
#[derive(Debug, Clone)]
pub struct IssueTemplate
{
    pub title_template: String,
    pub body_template: String,
}

impl IssueTemplate
{
    pub fn new(
        title_template: &str,
        body_template: &str,
    ) -> Self
    {
        Self {
            title_template: title_template.to_string(),
            body_template: body_template.to_string(),
        }
    }

    pub fn generate_issue(
        &self,
        title_data: &str,
        body_data: &str,
    ) -> Issue
    {
        Issue {
            title: self.title_template.replace("{title}", title_data),
            body: self.body_template.replace("{body}", body_data),
        }
    }
}
use ollama_rs::{
    generation::chat::{
        request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream,
    },
    models::LocalModel,
    Ollama,
};
use tokio::{
    io::{stdout, AsyncWriteExt},
    process::Command,
};
use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct Britney
{
    pub client: GithubClient,
    pub ollama: Ollama,
    desired_model: Option<String>,
}

impl Britney
{
    pub fn new() -> Self
    {
        let client = GithubClient::new();
        let ollama = Ollama::default();
        let desired_model = std::env::var("OLLAMA_MODEL").ok();
        Self {
            client,
            ollama,
            desired_model,
        }
    }

    fn is_britney_alive(
        &self,
        _models: Vec<LocalModel>,
    ) -> bool
    {
        for model in _models {
            if model.name == "britney" {
                return true;
            }
        }
        false
    }

    pub async fn check(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        println!("Checking...");
        let models = self.ollama.list_local_models().await.unwrap();
        if models.len() == 0 {
            Err("No models found. run `ollama run [model_name]` in a \
                 terminal to download a model.")?;
        }
        // check if Britney is in the models list
        let is_briney_alive = self.is_britney_alive(models);
        let modelfile = std::path::Path::new("./Modelfile");

        Ok(())
    }

    pub async fn generate_ai_content(&self) -> Vec<(String, String)>
    {
        unimplemented!()
    }
}
