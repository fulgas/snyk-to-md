use crate::model::security_report::{SecurityProject, SecurityReport};
use crate::parser::container::SnykContainer;
use crate::parser::Parser;
use anyhow::Context;
use anyhow::Result;
use std::borrow::Cow;
use std::fs;

pub struct SnykContainerParser;

impl Parser for SnykContainerParser {
    fn parse<'a>(&self, file_path: &str) -> Result<SecurityReport<'a>> {
        println!("Parsing Snyk Container report from file: {}", file_path);

        let json_content = fs::read_to_string(file_path)
            .with_context(|| format!("Could not read file: {}", file_path))?;

        let snyk_container: SnykContainer = serde_json::from_str(&json_content)
            .context("Failed to deserialize Snyk Container JSON.")?;

        let docker_image_project = SecurityProject {
            name: Cow::Owned(snyk_container.project_name),
            organization: Cow::Owned(snyk_container.org),
        };

        Ok(SecurityReport {
            projects: vec![docker_image_project],
        })
    }
}
