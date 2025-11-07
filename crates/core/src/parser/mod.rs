use crate::model::security_report::SecurityReport;
use anyhow::Result;
use std::str::FromStr;

mod code;
mod container;
pub mod factory;

#[derive(Clone, Debug)]
pub enum ParserType {
    Container,
    Code,
}

impl FromStr for ParserType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "container" => Ok(ParserType::Container),
            "code" => Ok(ParserType::Code),
            _ => Err(format!(
                "Invalid parser type: '{}'. Expected 'container' or 'code'",
                s
            )),
        }
    }
}

pub trait Parser {
    fn parse<'a>(&self, file_path: &str) -> Result<SecurityReport<'a>>;
}
