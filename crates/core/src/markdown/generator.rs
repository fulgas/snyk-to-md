use crate::model::security_report::SecurityReport;

pub trait MarkdownGenerator {
    fn generate_markdown_report(&self, report: &SecurityReport) -> anyhow::Result<String>;
}
