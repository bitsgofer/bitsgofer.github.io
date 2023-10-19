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
        .subcommand(Command::new("install")
            .about("Install necessary dependencies")
        )
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
        .subcommand(Command::new("serve")
            .about("Serve rendered content")
            .arg(
                arg!(web_dir: --web [WEB_DIR] "Directory of rendered content")
                    .default_value("_html")
            )
            .arg(
                arg!(bind_addr: --addr [BIND_ADDR] "Bind address (e.g: <IP:PORT>) for the server")
                    .default_value("127.0.0.1:7777")
            )
        )
        .subcommand(Command::new("render")
            .about("Render documents")
            .arg(
                arg!(content_dir: --content [CONTENT_DIR] "Directory with content")
                    .default_value("content")
            )
            .arg(
                arg!(theme_name: --theme [THEME] "Name of theme")
                    .default_value("personal_balance")
                    .value_parser(["personal_balance", "indie_studio"])
            )
            .arg(
                arg!(web_dir: --web [WEB_DIR] "Directory of rendered content")
                    .default_value("_html")
            )
        )
        .subcommand(Command::new("watch")
            .about("Watch for content changes, then render them")
            .arg(
                arg!(content_dir: --content [CONTENT_DIR] "Directory with content")
                    .default_value("content")
            )
            .arg(
                arg!(theme_name: --theme [THEME] "Name of theme")
                    .default_value("personal_balance")
                    .value_parser(["personal_balance", "indie_studio"])
            )
            .arg(
                arg!(web_dir: --web [WEB_DIR] "Directory of rendered content")
                    .default_value("_html")
            )
        )
}

mod install;
pub use install::install;
mod serve;
pub use serve::serve;
mod render;
pub use render::render;
mod watch;
pub use watch::watch;
mod create;
pub use create::create;
