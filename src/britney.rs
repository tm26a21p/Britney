use std::io::{stdout, Write};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::Command,
};
use tokio_stream::StreamExt;
use ollama_rs::{
    generation::chat::{
        request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream,
    },
    models::{create::CreateModelRequest, LocalModel},
    Ollama,
};

use crate::github_client::GithubClient;

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

    async fn opiniated_system_content(&self) -> String
    {
        format!(
            "You are Britney, a Github Issue content generator.
        Your only mission is to generate professional Github issues (title \
             and body).
             Always take in consideration the code provided previously by the \
             user.
             Use this template to create the issues by replacing the comments \
             by actual content:
             ### Feature Request

#### Summary

<!-- Provide a brief summary of the feature request -->

#### Motivation

<!-- Explain why this feature is needed and how it will benefit users -->

#### Detailed Description

<!-- Provide a detailed description of the feature, including any specific \
             requirements -->

#### Potential Solutions

<!-- Describe any potential solutions or approaches for implementing the \
             feature -->

#### Additional Context

<!-- Add any other context or screenshots about the feature request here -->

#### Alternatives Considered

<!-- Mention any alternatives you've considered and why they weren't suitable \
             -->

#### Related Issues

<!-- Reference any related issues or previous discussions -->
        ",
        )
    }

    async fn _followup_code(
        &self
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        Ok(self._open_and_read_file("src/github_client.rs").await?)
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
        contents = contents.replace(
            "You are Dolphin, a helpful AI assistant.",
            self.opiniated_system_content().await.as_str(),
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

    pub async fn generate_issue(
        &self
    ) -> Result<String, Box<dyn std::error::Error>>
    {
        let mut messages = vec![];
        let content = format!("Code: {}", self._followup_code().await?);
        let system_message: ChatMessage = ChatMessage::system(content.into());
        let user_message: ChatMessage = ChatMessage::user(
            "Create 1 complete issue about the code above.".into(),
        );
        messages.push(system_message);
        messages.push(user_message);

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
                // _ = stdout.flush();
                response += assistant_message.content.as_str();
            }
        }
        Ok(response)
    }
}
