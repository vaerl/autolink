use anyhow::Result;
use env_logger::Builder;
use link::Link;
use log::debug;
use log::LevelFilter;
use regex::Regex;
use std::{fs, path::PathBuf};
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
    // build from env to respect RUST_LOG
    let mut builder = Builder::from_default_env();

    let args: Args = Args::from_args();

    if args.verbose {
        println!("Setting the rust-debug-level to DEBUG.");
        builder.filter_level(LevelFilter::Debug);
    }
    builder.init();
    debug!("Starting operation.");

    println!("Extracting links from '{}'.", args.path.display());
    let links = find_links(&args.path, args.create_dirs)?;
    println!("Finished extracting links.");

    if args.delete {
        println!(
            "Deleting all symlinks specified by files in '{}'.",
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

fn find_links(path: &PathBuf, create_dirs: bool) -> Result<Vec<Link>> {
    let mut result = Vec::<Link>::new();
    if path.is_file() {
        println!("'{}' is file, adding link to list.", path.display());
        result.push(get_link(path, create_dirs)?)
    } else if path.is_dir() {
        println!(
            "'{}' is directory, finding links recursively.",
            path.display()
        );
        let paths = read_dir(path)?;
        for path in paths {
            result.append(&mut find_links(&path?.path(), create_dirs)?);
        }
    } else {
        println!(
            "Path '{}' is neither file nor directory, skipping.",
            path.display()
        );
    }
    Ok(result)
}

fn get_link(origin: &PathBuf, create_dirs: bool) -> Result<Link> {
    let reg = Regex::new(r"##!!(((~|.|..)?(/.+)+)|~)").unwrap();
    let mut destinations = Vec::<PathBuf>::new();

    debug!("Trying to open file '{}'.", origin.display());
    let file = File::open(&origin)?;
    for line in BufReader::new(file).lines() {
        let line = line?;

        for cap in reg.captures_iter(&line) {
            debug!("Matched: '{}', extracted substring: {}", &cap[0], &cap[1]);
            let matched_str = &cap[1];

            // expand tilde
            let path = if matched_str.contains("~") {
                debug!("Found tilde, replacing.");
                shellexpand::tilde(&matched_str).to_string()
            } else {
                matched_str.to_string()
            };

            let mut context = origin.parent().unwrap().to_path_buf();
            context.push(&path);
            debug!("Expanded path: {}", context.display());

            // NOTE canonicalize() fails when the path doesn't exist
            match context.canonicalize() {
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
