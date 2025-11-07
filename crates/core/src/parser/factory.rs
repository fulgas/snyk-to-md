use crate::parser::code::SnykCodeParser;
use crate::parser::container::container_parser::SnykContainerParser;
use crate::parser::{Parser, ParserType};

pub struct ParserFactory;

impl ParserFactory {
    pub fn create_parser(report_type: ParserType) -> anyhow::Result<Box<dyn Parser>> {
        match report_type {
            ParserType::Container => Ok(Box::new(SnykContainerParser)),
            ParserType::Code => Ok(Box::new(SnykCodeParser)),
        }
    }
}
