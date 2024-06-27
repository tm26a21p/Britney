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
            title: "New Issue".to_string(),
            body: String::new(),
        }
    }
}
