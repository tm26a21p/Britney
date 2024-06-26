use std::fs;

use crate::issue::*;

#[derive(Debug, Clone)]
pub struct IssueTemplate
{
    pub raw: String,
    pub title_template: String,
    pub body_template: String,
}

impl IssueTemplate
{
    // Load the template from the given path
    pub fn new(path: &str) -> Self
    {
        let content =
            fs::read_to_string(path).expect("Unable to read template file.");

        let mut title_template = String::new();
        let mut body_template = String::new();

        // Simple parsing logic assuming title and body are separated by "---"
        if let Some((title, body)) = content.split_once("---") {
            title_template = title.trim().to_string();
            body_template = body.trim().to_string();
        } else {
            println!("Invalid template formatting. Missing '---' separator");
        }

        Self {
            raw: content,
            title_template,
            body_template,
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
