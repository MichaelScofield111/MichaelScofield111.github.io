use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub published: bool,
    pub description: Option<String>,
    pub author: Option<String>,
    pub cover: Option<String>,
}
