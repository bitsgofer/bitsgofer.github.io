pub use std::str::FromStr;

/// Represent the type of supported documents.
#[derive(Debug, PartialEq)]
pub enum DocumentType {
    Markdown,
    GoogleDoc,
}
impl FromStr for DocumentType {
    type Err = ();

    fn from_str(input: &str) -> Result<DocumentType, Self::Err> {
        match input.to_lowercase().as_str() {
            "markdown" => Ok(DocumentType::Markdown),
            "googledoc" => Ok(DocumentType::GoogleDoc),
            _ => Err(()),
        }
    }
}

use clap::{arg, Command};

/// Create the CLI struct.
pub fn new_command() -> Command {
    Command::new("komet")
        .about("A utility for taking and publishing notes")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("install").about("Install necessary dependencies"))
        .subcommand(Command::new("create")
            .about("Create a document")
            .arg(
                arg!(document_type: <DOCUMENT_TYPE> "Type of the document")
                    .value_parser(["markdown", "googledoc"])
            )
            .arg(
                arg!(name: <NAME> "Name of document")
            )
            .arg(
                arg!(slug: --slug [SLUG] "Slug to use as document URI")
                    .default_value("")
            )
            .arg_required_else_help(true)
        )
        .subcommand(Command::new("serve").about("Serve rendered documents"))
        .subcommand(Command::new("render").about("Render documents"))
        .subcommand(Command::new("watch").about("Watch for content changes, then render them"))
}

mod install;
pub use install::install;

/// Serve rendered content.
pub fn serve() {
    println!("Run web server");
}

/// Render content.
pub fn render() {
    println!("Render content and prepare assets");
}
mod create;
pub use create::create;

/// Serve rendered content, then watch for changes and re-render.
pub fn watch() {
    println!("Watch for content changes, then render them");
}
