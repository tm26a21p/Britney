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
    models::{create::CreateModelRequest, LocalModel},
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

    fn alive(
        &self,
        models: &Vec<LocalModel>,
    ) -> bool
    {
        for model in models {
            if model.name == "britney" {
                return true;
            }
        }
        false
    }

    pub async fn spawn(
        &self,
        default: String,
    ) -> ()
    {
        let model = self.desired_model.clone().unwrap_or(default);

        let mut res = self
            .ollama
            .create_model_stream(CreateModelRequest::path(
                model,
                "./Modelfile".into(),
            ))
            .await
            .unwrap();

        while let Some(res) = res.next().await {
            let res = res.unwrap();
            println!("{:?}", res);
        }
    }

    async fn create_modelfile(
        &self,
        model: &str,
    ) -> Result<(), Box<dyn std::error::Error>>
    {
        // execute this command : ollama show --modelfile model > Modelfile
        let output = Command::new("ollama")
            .arg("show")
            .arg("--modelfile")
            .arg(format!("{}", model))
            .output()
            .await?;
        let mut file = tokio::fs::File::create("./Modelfile").await?;
        println!("content of Modelfile {:?}", output.stdout);
        file.write_all(&output.stdout).await?;
        Ok(())
    }

    pub async fn check(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        println!("Running a check check...");
        let models = self.ollama.list_local_models().await.unwrap();
        if models.len() == 0 {
            Err("No models found. run `ollama run [model_name]` in a \
                 terminal to download a model.")?;
        }
        if self.alive(&models) {
            println!("Britney is alive! She's gonna get so mad...");
            return Ok(());
        }
        if self.desired_model.is_none() {
            let fmodel = models[0].clone();
            println!(
                "No model specified detected. Using the first from the list: \
                 {}",
                fmodel.name
            );
            println!("Is there any Modelfile?");
            let modelfile = std::path::Path::new("./Modelfile");
            if !modelfile.exists() {
                println!("No Modelfile found. Creating one...");
                self.create_modelfile(&fmodel.name).await?;
            }
            println!("All good. Spawning Britney.");
            self.spawn(fmodel.name).await;
        }

        Ok(())
    }

    pub async fn generate_ai_content(&self) -> Vec<(String, String)>
    {
        unimplemented!()
    }
}
