use komet::*;

fn main() {
    let matches = komet::new_command().get_matches();

    match matches.subcommand() {
        Some(("install", _)) => {
            komet::install();
        }
        Some(("create", sub_matches)) => {
            let doctype = komet::DocumentType::from_str(sub_matches.get_one::<String>("DOCUMENT_TYPE").expect("required"));
            let name = sub_matches.get_one::<String>("NAME").expect("required");
            let slug = sub_matches.get_one::<String>("SLUG").expect("required"); // TODO: get from name
            komet::create(doctype.expect(""), name, slug);
        }
        Some(("serve", _)) => {
            komet::serve();
        }
        Some(("render", _)) => {
            komet::render();
        }
        Some(("watch", _)) => {
            komet::watch();
        }
        _ => unreachable!(),
    }
}
