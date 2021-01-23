use anyhow::Result;
use link::Link;
use log::debug;
use regex::Regex;
use std::path::PathBuf;
use std::{fs::read_dir, fs::File};
use std::{io::BufRead, io::BufReader};
use structopt::StructOpt;

mod link;
mod tests;

// using structopt auto-generates CLI-information
#[derive(StructOpt)]
struct Args {
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    #[structopt(short = "c", long = "create-dirs")]
    create_dirs: bool,

    #[structopt(short, long)]
    overwrite: bool,

    #[structopt(short, long)]
    delete: bool,

    #[structopt(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    debug!("Starting operation.");

    let args: Args = Args::from_args();

    if args.verbose {
        println!("Setting the rust-debug-level to DEBUG.");
        // FIXME this does nothing
        std::env::set_var("RUST_LOG", "debug");
    }

    println!("Extracting links from path {}.", args.path.display());
    let links = find_links(&args.path)?;
    println!("Finished extracting links.");

    if args.delete {
        println!(
            "Deleting all symlinks specified by files in {}.",
            args.path.display()
        );
        for link in links {
            link.delete()?;
        }
    } else {
        for link in links {
            link.link(args.overwrite, args.create_dirs)?;
        }
    }

    Ok(())
}

fn find_links(path: &PathBuf) -> Result<Vec<Link>> {
    let mut result = Vec::<Link>::new();
    if path.is_file() {
        println!("Path '{}' is file, adding link to list.", path.display());
        result.push(get_link(path)?)
    } else if path.is_dir() {
        println!(
            "Path '{}' is directory, finding links recursively.",
            path.display()
        );
        let paths = read_dir(path)?;
        for path in paths {
            result.append(&mut find_links(&path?.path())?);
        }
    } else {
        println!(
            "Path '{}' is neither file nor directory, skipping.",
            path.display()
        );
    }
    Ok(result)
}

fn get_link(origin: &PathBuf) -> Result<Link> {
    let reg = Regex::new(r"##!!([^\\B]+)\b").unwrap();
    let mut destinations = Vec::<PathBuf>::new();

    debug!("Trying to open file '{}'.", origin.display());
    let file = File::open(&origin)?;
    for line in BufReader::new(file).lines() {
        let line = line?;

        for cap in reg.captures_iter(&line) {
            debug!("Matched: '{}', extracted substring: {}", &cap[0], &cap[1]);

            // expand tilde
            let matched_str = &cap[1];
            let path = if matched_str.contains("~") {
                debug!("Found tilde, replacing.");
                shellexpand::tilde(&matched_str).to_string()
            } else {
                matched_str.to_string()
            };

            match PathBuf::from(&path).canonicalize() {
                Ok(mut destination) => {
                    destination.push(&origin.file_name().unwrap());
                    debug!("Origin: {}", &origin.display());
                    debug!("Destination: {}", destination.display());
                    destinations.push(destination);
                }
                Err(err) => println!("Path '{}' is not valid: {}", path, err),
            }
        }
    }

    Ok(Link {
        origin: origin.canonicalize()?,
        destinations,
    })
}
