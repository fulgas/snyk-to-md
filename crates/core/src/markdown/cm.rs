use crate::markdown::generator::MarkdownGenerator;
use crate::model::security_report::SecurityReport;
use askama::Template;

#[derive(Template)]
#[template(path = "report.md")]
pub struct ReportTemplate<'a> {
    projects: Vec<ReportProject<'a>>,
}

pub struct ReportProject<'a> {
    organization: &'a str,
    name: &'a str,
}

pub struct CommonMarkGenerator;

impl MarkdownGenerator for CommonMarkGenerator {
    fn generate_markdown_report(&self, report: &SecurityReport) -> anyhow::Result<String> {
        let template_ctx = ReportTemplate {
            projects: report
                .projects
                .iter()
                .map(|project| ReportProject {
                    name: &project.name,
                    organization: &project.organization,
                })
                .collect(),
        };

        template_ctx
            .render()
            .map_err(|e| anyhow::anyhow!("Rendering error: {}", e))
    }
}
