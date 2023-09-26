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
            "markdown"  => Ok(DocumentType::Markdown),
            "googledoc"  => Ok(DocumentType::GoogleDoc),
            _      => Err(()),
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
        .subcommand(
            Command::new("install")
                .about("Install necessary dependencies")
        )
        .subcommand(
            Command::new("create")
                .about("Create a document")
                .arg(arg!(<DOCUMENT_TYPE> "Type of the document"))
                .arg(arg!(<NAME> "Document name"))
                .arg(arg!([SLUG] "(Optional) Slug to use as document URI").default_value("")) // TODO: no default value?
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("serve")
                .about("Serve rendered documents")
        )
        .subcommand(
            Command::new("render")
                .about("Render documents")
        )
        .subcommand(
            Command::new("watch")
                .about("Watch for content changes, then render them")
        )
}

/// Install required dependencies.
pub fn install() {
    println!("Install pandoc and minify");
}

/// Create a document.
pub fn create(doctype: DocumentType, name: &str, slug: &str) {
    println!(
        "Creating new {:?} with name={} (slug={})",
        doctype, name, slug
    );
}

/// Serve rendered content.
pub fn serve() {
    println!("Run web server");
}

/// Render content.
pub fn render() {
    println!("Render content and prepare assets");
}

/// Serve rendered content, then watch for changes and re-render.
pub fn watch() {
    println!("Watch for content changes, then render them");
}
