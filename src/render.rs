/// Render content.
pub fn render(content_dir: &str, theme_name: &str, web_dir: &str) {
    println!(
        "Use theme '{}' to render content at '{}' into '{}'",
        theme_name, content_dir, web_dir
    );

    // use pandoc to render HTML pages
    my_theme_render_markdown(content_dir, web_dir);

    // render theme
    let mut theme_dir = PathBuf::from("themes");
    theme_dir.push(theme_name);
    let theme_dir = theme_dir.to_str().unwrap();
    render_my_theme(theme_dir, content_dir, web_dir);
}

pub fn render_my_theme(theme_dir: &str, content_dir: &str, web_dir: &str) {
    // create CSS assets
    let minified_css: PathBuf = [web_dir, "css", "theme.css"].iter().collect();
    match my_theme_minify_assets("css", theme_dir, minified_css.to_str().unwrap()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("cannot minify to {}; err= {}", minified_css.display(), e);
        }
    }
    // create JS assets
    let minified_js: PathBuf = [web_dir, "js", "theme.js"].iter().collect();
    match my_theme_minify_assets("js", theme_dir, minified_js.to_str().unwrap()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("cannot minify to {}; err= {}", minified_js.display(), e);
        }
    }

    match my_theme_copy_images("svg", theme_dir, web_dir) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("cannot copy {} files; err= {}", "svg", e);
        }
    }
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
        .arg("themes/personal_balance/pandoc/lua-filters/date-format.lua")
        .arg("--template")
        .arg("themes/personal_balance/templates/page.html")
        .arg("--highlight-style")
        .arg("themes/personal_balance/pandoc/highlight-theme/solarized.theme")
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

use std::vec::Vec;
fn my_theme_minify_assets(
    file_type: &str,
    theme_dir: &str,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // map MIME type => file extension to search for
    let extension = file_type;

    // collect files matching extensions so we can minify them
    let mut result: Vec<String> = Vec::new();
    for entry in WalkDir::new(theme_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_file() || path.extension() != Some(&Path::new(extension).as_os_str()) {
            continue;
        }
        result.push(String::from(path.to_str().unwrap()));
    }

    // The convention here is to lexically sort files before minify.
    // This allow things that are sensitive to declaration order (e.g: CSS)
    // to be minified correctly.
    result.sort();

    // run minify
    match Command::new("minify")
        .arg("--type")
        .arg(file_type)
        .arg("--bundle")
        .arg("--output")
        .arg(output)
        .args(result)
        .output()
    {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::from(e));
        }
    }

    Ok(())
}

use std::fs;
fn my_theme_copy_images(
    file_type: &str,
    theme_dir: &str,
    web_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // map MIME type => file extension to search for
    let extension = file_type;

    // collect files matching extensions so we can minify them
    for entry in WalkDir::new(theme_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if !path.is_file() || path.extension() != Some(&Path::new(extension).as_os_str()) {
            continue;
        }

        let mut output = PathBuf::from(web_dir);
        output.push(path.strip_prefix(theme_dir).ok().unwrap());
        let output = output.to_str().unwrap();

        let src = path.to_str().unwrap();
        fs::copy(src, output)?;
    }

    Ok(())
}
