use anyhow::Context;
use clap::Parser;
use snyk_to_md_core::parser::factory::ParserFactory;
use std::fs;

use crate::cli::Cli;
use snyk_to_md_core::markdown::factory::{MarkdownFormat, MarkdownGeneratorFactory};
use snyk_to_md_core::model::security_report::SecurityReport;

mod cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("🔎 Processing report: {}", cli.input.display());
    println!("📋 Detected report type: {:?}\n", cli.parser_type);

    let parser = ParserFactory::create_parser(cli.parser_type)?;
    let report: SecurityReport = parser.parse(&cli.input.to_string_lossy())?;

    println!("📝 Generating report...");

    let markdown_format = MarkdownFormat::CommonMark;
    let generator = MarkdownGeneratorFactory::create_generator(markdown_format);

    let markdown_report = generator.generate_markdown_report(&report)?;

    match cli.output {
        Some(output_path) => {
            fs::write(&output_path, markdown_report).with_context(|| {
                format!("Failed to write report to file: {}", output_path.display())
            })?;
            println!(
                "\n✅ Report successfully saved to: {}",
                output_path.display()
            );
        }
        None => {
            println!("\n--- Generated Report ---\n");
            println!("{}", markdown_report);
        }
    }
    Ok(())
}
