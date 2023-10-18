/// Render content.
pub fn render(content_dir: &str, theme_name: &str, web_dir: &str) {
    println!(
        "Use theme '{}' to render content at '{}' into '{}'",
        theme_name, content_dir, web_dir
    );

    render_my_theme(content_dir, web_dir);
}

pub fn render_my_theme(content_dir: &str, web_dir: &str) {
    // use pandoc to render HTML pages
    my_theme_render_markdown(content_dir, web_dir);

    // create CSS assets
    // create JS assets
    // create images
    return;
}

use std::path::{Path, PathBuf};
use walkdir::WalkDir;
fn my_theme_render_markdown(content_dir: &str, web_dir: &str) {
    for entry in WalkDir::new(content_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_file() || path.file_name() != Some(&Path::new("README.md").as_os_str()) {
            continue;
        }

        match path.strip_prefix(content_dir) {
            Err(e) => {
                eprintln!("cannot strip prefix {} {}", content_dir, e);
            }
            Ok(p) => {
                let parent = p.parent().unwrap();
                let mut output = PathBuf::from(web_dir);
                output.push(parent);
                output.push("index.html");

                println!("pandoc: {} -> {}", path.display(), output.display());
                pandoc_render(path.to_str().unwrap(), output.to_str().unwrap()).unwrap()
            }
        }
    }
}

use std::process::Command;
fn pandoc_render(markdown: &str, html: &str) -> Result<(), Box<dyn std::error::Error>> {
    match Command::new("pandoc")
        .arg("--standalone")
        .arg("--output")
        .arg(html)
        //  .args("--metadata", "current-date=...")
        //  .args("--metadata", "last-modified-date=...")
        .arg("--lua-filter")
        .arg("themes/indie_studio/pandoc/lua-filters/date-format.lua")
        .arg("--template")
        .arg("themes/indie_studio/templates/page.html")
        .arg("--highlight-style")
        .arg("themes/indie_studio/pandoc/highlight-theme/solarized.theme")
        .arg(markdown)
        .output()
    {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::from(e));
        }
    }

    Ok(())
}
