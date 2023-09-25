use clap::{arg, Command};

fn cli() -> Command {
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

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("install", _)) => {
            println!("Install pandoc and minify");
        }
        Some(("create", sub_matches)) => {
            let doctype = sub_matches.get_one::<String>("DOCUMENT_TYPE").expect("required");
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            let slug = sub_matches.get_one::<String>("SLUG").expect("required"); // TODO: get from name

            println!(
                "Creating new {} with name={} (slug={})",
                doctype, name, slug
            );
        }
        Some(("serve", _)) => {
            println!("Run web server");
        }
        Some(("render", _)) => {
            println!("Render content and prepare assets");
        }
        Some(("watch", _)) => {
            println!("Watch for content changes, then render them");
        }
        _ => unreachable!(),
    }
}
