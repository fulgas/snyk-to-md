use crate::markdown::cm::CommonMarkGenerator;
use crate::markdown::generator::MarkdownGenerator;
use std::fmt::Display;

pub struct MarkdownGeneratorFactory;

impl MarkdownGeneratorFactory {
    pub fn create_generator(format: MarkdownFormat) -> Box<dyn MarkdownGenerator> {
        match format {
            MarkdownFormat::CommonMark => Box::new(CommonMarkGenerator),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MarkdownFormat {
    CommonMark,
}

impl Display for MarkdownFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkdownFormat::CommonMark => write!(f, "cm"),
        }
    }
}
