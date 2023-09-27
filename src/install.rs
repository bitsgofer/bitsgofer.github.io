use reqwest;
use std::fs::File;
use std::io::Write;
fn download_file(url: &str, dest_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let mut response = client.get(url).send()?;

    if response.status().is_success() {
        let mut dest = File::create(dest_path)?;
        std::io::copy(&mut response, &mut dest)?;
        dest.flush()?;
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to download! HTTP Error: {:?}", response.status()),
        )));
    }

    println!("Downloaded file from '{}' to {}", url, dest_path);
    Ok(())
}

use std::process::Command;
fn install_deb_package(deb_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    match Command::new("sudo")
        .arg("dpkg")
        .arg("--install")
        .arg(deb_path)
        .output()
    {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::from(e));
        }
    };

    println!("Installed package: {}", deb_path);
    Ok(())
}

use home;
fn install_gzipped_binary(gzip_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = home::home_dir().expect("no $HOME dir");
    let bin_dir = home_dir.join(".local/bin");
    let dir_path = bin_dir.to_str().expect("");

    match Command::new("tar")
        .arg("--extract")
        .arg("--gzip")
        .arg("--directory")
        .arg(dir_path)
        .arg("--file")
        .arg(gzip_path)
        .output()
    {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::from(e));
        }
    };

    println!("Installed gzipped binary: {}", gzip_path);
    Ok(())
}

use tempfile;
fn install_pandoc() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let deb = dir.path().join("pandoc.deb");
    let tmp_file = deb.to_str().expect("");

    match download_file(
        "https://github.com/jgm/pandoc/releases/download/3.1.8/pandoc-3.1.8-1-amd64.deb",
        tmp_file,
    ) {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::from(e));
        }
    };

    match install_deb_package(tmp_file) {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(Box::from(e));
        }
    }
}

fn install_minify() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let gzip = dir.path().join("minify.tar.gz");
    let tmp_file = gzip.to_str().expect("");

    match download_file(
        "https://github.com/tdewolff/minify/releases/download/v2.12.8/minify_linux_amd64.tar.gz",
        tmp_file,
    ) {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::from(e));
        }
    };

    match install_gzipped_binary(tmp_file) {
        Ok(_) => Ok(()),
        Err(e) => {
            return Err(Box::from(e));
        }
    }
}

use os_info;
/// Install required dependencies.
pub fn install() {
    let info = os_info::get();
    let os_type = info.os_type();
    let arch = info.architecture();

    let system = (os_type, arch);
    match system {
        (os_info::Type::Ubuntu, Some("x86_64")) | (os_info::Type::Debian, Some("x86_64")) => {
            install_dependencies().expect("cannot install pandoc");
        }
        _ => {
            println!(
                "The current system (OS= {:?}, architecture= {:?}) is not supported",
                os_type, arch
            );
        }
    }
}

fn install_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    match install_pandoc() {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::from(e));
        }
    };

    match install_minify() {
        Ok(_) => (),
        Err(e) => {
            return Err(Box::from(e));
        }
    };

    Ok(())
}
