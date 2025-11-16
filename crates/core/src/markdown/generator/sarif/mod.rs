mod converter;
mod generator;
mod types;

use crate::markdown::generator::sarif::generator::SarifMarkdownGenerator;
use crate::markdown::generator::{MarkdownFormatFactory, MarkdownGenerator};
use crate::markdown::MarkdownFormat;

pub(crate) struct SarifMarkdownGeneratorFactory;

impl MarkdownFormatFactory for SarifMarkdownGeneratorFactory {
    fn create_generator(
        &self,
        markdown_format: MarkdownFormat,
        with_emoji: bool,
    ) -> Box<dyn MarkdownGenerator> {
        Box::new(SarifMarkdownGenerator::new(markdown_format, with_emoji))
    }
}
