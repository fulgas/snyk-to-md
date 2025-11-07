use std::borrow::Cow;

#[derive(Debug)]
pub struct SecurityReport<'a> {
    pub projects: Vec<SecurityProject<'a>>,
}

#[derive(Debug)]
pub struct SecurityProject<'a> {
    pub name: Cow<'a, str>,
    pub organization: Cow<'a, str>,
}

#[derive(Debug)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}
