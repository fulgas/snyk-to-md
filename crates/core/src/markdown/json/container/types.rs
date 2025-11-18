use askama::Template;
use serde_snyk_container::Severity;
use std::fmt;

#[derive(Template)]
#[template(path = "json/report.md")]
pub(in crate::markdown) struct ReportTemplate {
    pub(in crate::markdown) projects: Vec<ReportProject>,
    pub(in crate::markdown) timestamp: String,
    pub(in crate::markdown) with_emoji: bool,
    #[allow(dead_code)]
    pub(in crate::markdown) is_gfm: bool,
}
pub(in crate::markdown) struct ReportProject {
    pub(in crate::markdown) name: String,
    pub(in crate::markdown) organization: String,
    pub(in crate::markdown) project_type: ReportProjectType,
    pub(in crate::markdown) target_file: String,
    pub(in crate::markdown) summary: ReportSummary,
    pub(in crate::markdown) vulnerabilities: Vec<ReportVulnerability>,
}

pub(in crate::markdown) enum ReportProjectType {
    DockerImage,
    Application,
}

#[derive(Clone, Default)]
pub(in crate::markdown) struct ReportSummary {
    pub(in crate::markdown) critical: usize,
    pub(in crate::markdown) high: usize,
    pub(in crate::markdown) medium: usize,
    pub(in crate::markdown) low: usize,
    pub(in crate::markdown) unique_count: usize,
}

#[derive(PartialEq, Ord, Eq, PartialOrd)]
pub(in crate::markdown) enum ReportSeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl From<Severity> for ReportSeverity {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Low => ReportSeverity::Low,
            Severity::Medium => ReportSeverity::Medium,
            Severity::High => ReportSeverity::High,
            Severity::Critical => ReportSeverity::Critical,
        }
    }
}

impl fmt::Display for ReportSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportSeverity::Critical => write!(f, "Critical"),
            ReportSeverity::High => write!(f, "High"),
            ReportSeverity::Medium => write!(f, "Medium"),
            ReportSeverity::Low => write!(f, "Low"),
        }
    }
}

pub(in crate::markdown) struct ReportVulnerability {
    pub(in crate::markdown) id: String,
    pub(in crate::markdown) title: String,
    pub(in crate::markdown) severity: ReportSeverity,
    pub(in crate::markdown) package_name: String,
    pub(in crate::markdown) version: String,
    pub(in crate::markdown) cvss_score: Option<f64>,
    pub(in crate::markdown) is_upgradable: bool,
    pub(in crate::markdown) is_patchable: bool,
    pub(in crate::markdown) cve_ids: Vec<String>,
    pub(in crate::markdown) from_paths: Vec<Vec<String>>,
}
