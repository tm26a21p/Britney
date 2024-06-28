use std::{
    fs,
    io::{stdout, Write},
};

use tokio::{io::AsyncWriteExt, process::Command};
use tokio_stream::StreamExt;
use ollama_rs::{
    generation::chat::{
        request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream,
        MessageRole,
    },
    models::{create::CreateModelRequest, LocalModel},
    Ollama,
};

use crate::{
    github_client::GithubClient, issue::Issue, issue_template::IssueTemplate,
};

#[derive(Debug)]
pub struct Britney
{
    pub client: GithubClient,
    ollama: Ollama,
    desired_model: Option<String>,
    it: IssueTemplate,
}

impl Britney
{
    pub fn new(template_path: Option<&str>) -> Self
    {
        let client = GithubClient::new();
        let ollama = Ollama::default();
        let desired_model = std::env::var("OLLAMA_MODEL").ok();
        let path = template_path.unwrap_or("issue_templates/default.md");
        let template = IssueTemplate::new(path);

        Self {
            client,
            ollama,
            desired_model,
            it: template,
        }
    }

    fn alive(
        &self,
        models: &Vec<LocalModel>,
    ) -> bool
    {
        for model in models {
            if model.name.contains("Britney") {
                return true;
            }
        }
        false
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

        let mut stdout = stdout();
        while let Some(res) = res.next().await {
            let res = res.unwrap();
            _ = stdout.write_all(res.message.as_bytes());
            // stdout.flush().unwrap();
        }
    }

    async fn behavior(
        &self,
        template: String,
    ) -> String
    {
        format!(
            "You are Britney, a Github Issues generator. 
            You create some professional and complete GitHub Issue, 100% \
             based on the following template:

                {template}

            Adjust the template as needed for each issue. 
            Your answer is only the issue asked.
            Don't provide the title in markdown format, just the body.
            ",
        )
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

        let mut contents = String::from_utf8(output.stdout)?;
        let template = self.it.raw.to_owned();
        contents = contents.replace(
            "You are Dolphin, a helpful AI assistant.",
            self.behavior(template).await.as_str(),
        );
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

    fn add_message(
        &self,
        messages: &mut Vec<ChatMessage>,
        content: String,
        role: MessageRole,
    )
    {
        let message: ChatMessage = match role {
            MessageRole::User => ChatMessage::user(content.into()),
            MessageRole::System => ChatMessage::system(content.into()),
            MessageRole::Assistant => ChatMessage::assistant(content.into()),
        };
        messages.push(message);
    }

    fn parse_response(
        &self,
        response: String,
    ) -> Issue
    {
        println!("OG Response: {}", response);
        let mut title = String::new();
        let mut body = String::new();
        let mut is_title = true;

        for line in response.lines() {
            if line.is_empty() {
                continue;
            }

            if is_title {
                title = line.to_string();
                is_title = false;
            } else {
                body += line;
                body += "\n";
            }
        }

        Issue { title, body }
    }

    pub async fn generate_issue_from_file(
        &self,
        path: &str,
    ) -> Result<Issue, Box<dyn std::error::Error>>
    {
        let mut messages = vec![];

        let code = fs::read_to_string(path).expect("Unable to read file");
        let content = self.behavior(self.it.raw.to_owned()).await;
        self.add_message(&mut messages, content, MessageRole::System);
        let content = format!("{}: {}", path, code);
        self.add_message(&mut messages, content, MessageRole::System);
        let content = "Produce a professional Github Issue based on the code \
                       provided."
            .into();
        self.add_message(&mut messages, content, MessageRole::User);

        let mut stream: ChatMessageResponseStream = self
            .ollama
            .send_chat_messages_stream(ChatMessageRequest::new(
                "Britney".to_string(),
                messages.to_owned(),
            ))
            .await?;
        let mut response = String::new();
        while let Some(Ok(res)) = stream.next().await {
            if let Some(assistant_message) = res.message {
                response += assistant_message.content.as_str();
            }
        }
        Ok(self.parse_response(response))
    }

    pub async fn post_issue(
        &self,
        issue: Issue,
    ) -> Result<(), Box<dyn std::error::Error>>
    {
        self.client.create_issue(issue).await?;
        Ok(())
    }
}
