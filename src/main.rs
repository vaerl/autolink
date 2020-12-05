use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;
use structopt::StructOpt;

// using structopt auto-generates CLI-information
#[derive(StructOpt)]
struct Arg {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

// TODO eliminate unwrap if possible
// TODO check return-types
// TODO handle multiple links?
// TODO save created symlinks for deletion
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
        warn!(
            "Path {} is neither file nor directory, skipping.",
            origin.display()
        );
        Ok(())
    }
}

fn link_file(mut origin: PathBuf) -> Result<()> {
    // open file from path
    let content: String = std::fs::read_to_string(&origin)
        .with_context(|| format!("Could not read file `{}`", &origin.display()))?;

    for line in content.lines() {
        // TODO use starts_with?
        if line.contains("##!!") {
            // TODO optimize -> use bufReader?
            debug!("Recognized tag: {}", line);

            // cut indicator from line, convert to path
            let substr: String = line.chars().into_iter().skip(4).collect();
            debug!("Extracted substring: {}", substr);

            // check if specified destination exists
            let mut destination: PathBuf = PathBuf::from(substr);
            if !destination.exists() {
                warn!(
                    "Destination {} does not exist, skipping.",
                    destination.display()
                );
                continue;
            }

            // get absolute path for destination and origin
            destination = destination.canonicalize()?;
            destination.push(&origin.file_name().unwrap());
            origin = origin.canonicalize()?;
            debug!("Origin: {}", &origin.display());
            debug!("Destination: {}", destination.display());

            // symlink from the given path to destination
            match symlink(&origin, destination) {
                Ok(res) => info!("Result: {:?}", res),
                Err(err) => error!("Symlink-Error: {}", err),
            }
        }
    }
    Ok(())
}
