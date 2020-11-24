use anyhow::{Context, Result};
use log::{debug, info, warn};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

// using structopt auto-generates CLI-information
#[derive(StructOpt)]
struct Arg {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

// TODO eliminate unwrap if possible
// TODO check return-types
fn main() -> Result<()> {
    env_logger::init();
    info!("Starting.");

    let arg = Arg::from_args();
    link(arg.path)
}

fn link(origin: PathBuf) -> Result<()> {
    if origin.is_file() {
        info!("Path {} is file, trying to symlink.", origin.display());
        link_file(origin)
    } else if origin.is_dir() {
        info!(
            "Path {} is directory, trying to symlink contents recursively.",
            origin.display()
        );
        let paths = fs::read_dir(origin).unwrap();
        for path in paths {
            link(path.unwrap().path());
        }
        return Ok(());
    } else {
        Ok(())
    }
}

fn link_file(origin: PathBuf) -> Result<()> {
    // open file from path
    let content: String = std::fs::read_to_string(&origin)
        .with_context(|| format!("Could not read file `{}`", &origin.display()))?;

    for line in content.lines() {
        if line.contains("##!!") {
            // TODO optimize -> use bufReader?
            debug!("Recognized tag: {}", line);

            // cut indicator from line, convert to path
            let substr: String = line.chars().into_iter().skip(4).collect();
            let destination: PathBuf = PathBuf::from(substr);
            if !destination.exists() {
                warn!(
                    "Destination {} does not exist, skipping.",
                    destination.display()
                );
                continue;
            }

            let mut full_origin: PathBuf = PathBuf::from(&destination);
            full_origin.push(origin.file_name().unwrap());
            if link_exists(full_origin) {
                warn!("Link exists at {}, skipping.", destination.display());
                continue;
            }

            // symlink from the given path to destination
            // TODO convert to complete paths
            Command::new("ln")
                .arg("-s")
                .arg(&origin.canonicalize().unwrap())
                .arg(destination.canonicalize().unwrap())
                .spawn()
                .expect("Could not symlink.");
        }
    }
    Ok(())
}

fn link_exists(path: PathBuf) -> bool {
    info!("Testing if symlink {} exists.", path.display());
    match Command::new("test").arg("-f").arg(&path).output() {
        Ok(output) => output.status.success(),
        Err(err) => {
            debug!("Something went wrong, returning false: {}", err);
            false
        }
    }
}
