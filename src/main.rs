use komet::*;

fn main() {
    let matches = komet::new_command().get_matches();

    match matches.subcommand() {
        Some(("install", _)) => {
            komet::install();
        }
        Some(("create", sub_matches)) => {
            let doctype_arg = sub_matches.get_one::<String>("document_type").unwrap();
            let doctype = komet::DocumentType::from_str(doctype_arg).unwrap();
            let name = sub_matches.get_one::<String>("name").unwrap();
            let slug_opt = sub_matches.get_one::<String>("slug").unwrap();
            let slug = if slug_opt != "" { slug_opt } else { name };
            komet::create(doctype, name, slug);
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
