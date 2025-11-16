use crate::markdown::generator::sarif::converter::convert_sarif_to_view;
use crate::markdown::generator::sarif::types::SarifReportTemplate;
use crate::markdown::generator::{GeneratorError, MarkdownGenerator};
use crate::markdown::MarkdownFormat;
use crate::parser::ParsedReport;
use askama::Template;

pub(crate) struct SarifMarkdownGenerator {
    markdown_format: MarkdownFormat,
    with_emoji: bool,
}

impl SarifMarkdownGenerator {
    pub(crate) fn new(markdown_format: MarkdownFormat, with_emoji: bool) -> Self {
        Self {
            markdown_format,
            with_emoji,
        }
    }
}

impl MarkdownGenerator for SarifMarkdownGenerator {
    fn generate_markdown_template(
        &self,
        parsed_report: &ParsedReport,
    ) -> Result<String, GeneratorError> {
        let sarif = match parsed_report {
            ParsedReport::Sarif(s) => s,
            _ => {
                return Err(GeneratorError::AskamaError(askama::Error::Custom(
                    "Expected SARIF report".into(),
                )))
            }
        };

        let runs = convert_sarif_to_view(sarif);
        let timestamp = chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();

        let is_gfm = matches!(self.markdown_format, MarkdownFormat::GitHubFlavored);

        let template = SarifReportTemplate {
            runs,
            timestamp,
            with_emoji: self.with_emoji,
            is_gfm,
        };

        template.render().map_err(GeneratorError::AskamaError)
    }
}
