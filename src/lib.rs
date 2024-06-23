use std::io::{stdout, Write};

use dotenv::dotenv;
use octocrab::Octocrab;
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

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
use tokio::{io::AsyncReadExt, process::Command};
// use tokio_stream::StreamExt;

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
            // println!("Model: {:?}", model);
            if model.name.contains("Britney") {
                return true;
            }
        }
        false
    }

    async fn _open_and_read_file(
        &self,
        file_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        let mut file = tokio::fs::File::open(file_path).await?;
        let mut contents: String = String::new();
        file.read_to_string(&mut contents).await?;
        Ok(contents)
    }

    pub async fn spawn(&self)
    {
        let modelfile_path = std::env::var("MODELFILE_PATH")
            .unwrap_or("./Modelfile".to_owned());
        let mut res = self
            .ollama
            .create_model_stream(CreateModelRequest::path(
                "Britney".into(),
                modelfile_path.to_string(),
            ))
            .await
            .unwrap();

        while let Some(res) = res.next().await {
            let res = res.unwrap();
            println!("{:?}", res);
        }
    }

    pub fn system(
        &self,
        d: &str,
    ) -> String
    {
        format!("SYSTEM \"{}\n\"", d)
    }

    async fn opiniated_system_content(&self) -> String
    {
        format!(
            "You are Britney.
        Your only mission is to help generating Github issues.
        The only answer you need to provide is `title` and `body` of the \
             issue.
        Based on this code that will be provided, create as many issues as \
             you can, Britney!
        ",
        )
    }

    async fn followup_code(&self)
        -> Result<String, Box<dyn std::error::Error>>
    {
        let main = self._open_and_read_file("src/main.rs").await?;
        let lib = self._open_and_read_file("src/lib.rs").await?;
        Ok(format!("```rust\n{}\n{}\n```", main, lib))
    }

    async fn create_modelfile(
        &self,
        model: &str,
    ) -> Result<(), Box<dyn std::error::Error>>
    {
        // Execute this command: ollama show --modelfile model > Modelfile
        let output = Command::new("ollama")
            .arg("show")
            .arg("--modelfile")
            .arg(model)
            .output()
            .await?;

        // Read the contents of the output
        let mut contents = String::from_utf8(output.stdout)?;
        // let a = self.system("You are Britney, a helpful AI assistant.");
        contents = contents.replace(
            "You are Dolphin, a helpful AI assistant.",
            self.opiniated_system_content().await.as_str(),
        );
        // Write the modified contents to the file
        let mut file = tokio::fs::File::create("./Modelfile").await?;
        file.write_all(contents.as_bytes()).await?;
        Ok(())
    }

    async fn check_modelfile(
        &self,
        choice: String,
    ) -> Result<(), Box<dyn std::error::Error>>
    {
        println!("Is there any Modelfile? We need one to run Britney.");
        let modelfile = std::path::Path::new("Modelfile");
        if !modelfile.exists() {
            println!("No Modelfile found. Creating one based on {}", choice);
            self.create_modelfile(&choice).await?;
        }
        Ok(())
    }

    fn model_choice(
        &self,
        models: &Vec<LocalModel>,
    ) -> String
    {
        let mut choice = models[0].name.to_owned();

        if self.desired_model.is_none() {
            println!(
                "No model specified. Using the first from the list: {}",
                choice
            );
        } else {
            choice = self.desired_model.to_owned().unwrap();
        }
        choice
    }

    pub async fn check(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        println!("Running a check...");
        let models = self.ollama.list_local_models().await.unwrap();
        if models.is_empty() {
            return Err("No models found. Run `ollama run [model_name]` in \
                        a terminal to download a model."
                .into());
        }

        // println!("Available models: {:?}", models);
        if self.alive(&models) {
            println!("Britney is alive! She's gonna get so mad...");
            return Ok(());
        }

        println!("Britney is not alive. Let's fix that.");
        let model_choice = self.model_choice(&models);
        _ = self.check_modelfile(model_choice.clone()).await;
        self.spawn().await;

        Ok(())
    }

    pub async fn answer(
        &self,
        q: &str,
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        let mut messages = vec![];
        let system_message =
            ChatMessage::user(self.followup_code().await.unwrap());
        messages.push(system_message);
        let user_message = ChatMessage::user(q.to_string());
        messages.push(user_message);

        println!("messages: {:?}", messages);
        let mut stream: ChatMessageResponseStream = self
            .ollama
            .send_chat_messages_stream(ChatMessageRequest::new(
                "Britney".to_string(),
                messages.to_owned(),
            ))
            .await?;
        let mut stdout = stdout();
        let mut response = String::new();
        while let Some(Ok(res)) = stream.next().await {
            if let Some(assistant_message) = res.message {
                _ = stdout.write_all(assistant_message.content.as_bytes());
                _ = stdout.flush();
                response += assistant_message.content.as_str();
            }
        }
        println!("response: {}", response);
        Ok(response)
    }

    pub async fn generate_issues(&self) -> Vec<(String, String)>
    {
        vec![]
    }
}
