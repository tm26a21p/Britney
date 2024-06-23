use crate::github_client::Issue;

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
