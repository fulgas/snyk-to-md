use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Failed to generate markdown: {0}")]
    GeneratorError(#[from] GeneratorError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("Template error: {0}")]
    TemplateError(#[from] askama::Error),
}

#[derive(Debug, Error)]
pub enum BuilderError {
    #[error("Content was not provided")]
    MissingContent,
}
