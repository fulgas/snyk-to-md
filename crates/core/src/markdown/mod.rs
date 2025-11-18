use crate::error::GeneratorError;
use serde::de::DeserializeOwned;

pub mod json;
pub mod sarif;

#[derive(Debug, Clone, Copy)]
pub enum MarkdownFormat {
    CommonMark,
    GitHubFlavored,
}

pub trait MarkdownGenerator {
    type Input: DeserializeOwned;
    fn generate_markdown_template(&self, report: &Self::Input) -> Result<String, GeneratorError>;
}
