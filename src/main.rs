use anyhow::Result;
use log::{debug, error, info, warn};
use regex::Regex;
use std::path::PathBuf;
use std::{fs, io::BufRead, io::BufReader};
use std::{fs::File, os::unix::fs::symlink};
use structopt::StructOpt;

mod tests;

// using structopt auto-generates CLI-information
#[derive(StructOpt)]
struct Arg {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}
fn main() -> Result<()> {
    env_logger::init();
    info!("Starting.");

    // NOTE using a struct-opt might prove beneficial later ¯\_(ツ)_/¯
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
        let paths = fs::read_dir(origin)?;
        for path in paths {
            link(path?.path())?;
        }
        Ok(())
    } else {
        warn!(
            "Path {} is neither file nor directory, skipping.",
            origin.display()
        );
        Ok(())
    }
}

fn link_file(mut origin: PathBuf) -> Result<()> {
    let file = File::open(&origin)?;
    for line in BufReader::new(file).lines() {
        let line = line?;

        let re = Regex::new(r"##!!([^\\B]+)\b").unwrap();

        for cap in re.captures_iter(&line) {
            debug!("Matched: {}, extracted substring: {}", &cap[0], &cap[1]);

            // check if specified destination exists
            let mut destination: PathBuf = PathBuf::from(&cap[1]);
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
                Ok(res) => debug!("Result: {:?}", res),
                Err(err) => error!("Symlink-Error: {}", err),
            }
        }
    }
    Ok(())
}
