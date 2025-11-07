use clap::Parser;
use snyk_to_md_core::parser::ParserType;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "snyk-to-md")]
#[command(about = "A tool to generate security reports from various snyk JSON files.", long_about = None
)]
#[command(version)]
pub struct Cli {
    #[arg(short, long)]
    pub input: PathBuf,

    #[arg(short, long, value_parser)]
    pub parser_type: ParserType,

    #[arg(short, long)]
    pub output: Option<PathBuf>,
}
