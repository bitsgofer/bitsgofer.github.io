use notify::event::ModifyKind;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
/// Render content.
use std::path::Path;
pub fn watch(content_dir: &str, theme_name: &str, web_dir: &str) {
    println!("Watch for content changes, then render them");

    if let Err(error) = watch_content_dir(content_dir, theme_name, web_dir) {
        eprintln!("watch err: {}", error);
    }

    println!("Done");
}

use crate::render;
fn watch_content_dir(content_dir: &str, theme_name: &str, web_dir: &str) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new(content_dir), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => match event.kind {
                EventKind::Modify(ModifyKind::Data(_)) => {
                    render::render(content_dir, theme_name, web_dir);
                    println!("Rendered: {:#?}", event.paths);
                }
                _ => {
                    continue;
                }
            },
            Err(error) => eprintln!("Error: {error:?}"),
        }
    }

    Ok(())
}
