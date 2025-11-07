use crate::model::security_report::SecurityReport;
use crate::parser::Parser;
use anyhow::Result;

pub struct SnykCodeParser;

impl Parser for SnykCodeParser {
    fn parse<'a>(&self, _: &str) -> Result<SecurityReport<'a>> {
        Ok(SecurityReport { projects: vec![] })
    }
}
