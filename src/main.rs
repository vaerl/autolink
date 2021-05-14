use anyhow::Result;
use human_panic::setup_panic;
use link::Link;
use path_absolutize::Absolutize;
use regex::Regex;
use std::path::PathBuf;
use std::{fs::read_dir, fs::File};
use std::{io::BufRead, io::BufReader};
use structopt::StructOpt;

mod link;
mod tests;

// using structopt auto-generates CLI-information
// NOTE doc-comments are automatically displayed when using -h
#[derive(StructOpt)]
#[structopt(name = "autolink", about = "Automatically symlink files.")]
struct Autolink {
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Create folders that do not exist yet
    #[structopt(short = "c", long = "create")]
    create_dirs: bool,

    /// Delete existing symlinks and add again, effectively overwriting existing links
    #[structopt(short, long)]
    overwrite: bool,

    /// Delete all symlinks specified in the provided folder/file
    #[structopt(short, long)]
    delete: bool,

    /// Show verbose logs
    #[structopt(short, long)]
    verbose: bool,
}

impl Autolink {
    /// Start the specified operation.
    fn do_op(&self) -> Result<()> {
        let links = self.find_links(&self.path)?;
        Autolink::spacer();

        if self.delete {
            self.log(
                format!("Removing all symlinks from {}", self.path.display()),
                1,
            );
            for link in links {
                link.delete()?;
            }
        } else {
            self.log(
                format!("Symlinking all files in {}", self.path.display()),
                1,
            );
            for link in links {
                link.link(self.overwrite, self.create_dirs)?;
            }
        }
        Ok(())
    }

    // LINKS

    /// Find all links recursively in a given folder or in a file.
    fn find_links(&self, path: &PathBuf) -> Result<Vec<Link>> {
        let mut links = Vec::<Link>::new();
        if path.is_file() {
            self.log(format!("Getting links from {}", path.display()), 1);
            links.push(self.get_link(path)?)
        } else if path.is_dir() {
            self.log(format!("Checking directory {}", path.display()), 1);
            let paths = read_dir(path)?;
            for path in paths {
                links.append(&mut self.find_links(&path?.path())?);
            }
        } else {
            self.log(format!("Skipping {}", path.display()), 1);
        }
        Ok(links)
    }

    /// Get all links from a file.
    fn get_link(&self, origin: &PathBuf) -> Result<Link> {
        let reg = Regex::new(r"##!!(((~|.|..)?(/.+)+)|~)").unwrap();
        let mut destinations = Vec::<PathBuf>::new();

        self.verbose(format!("Trying to open file {}", origin.display()), 3);
        let file = File::open(&origin)?;
        for line in BufReader::new(file).lines() {
            let line = line?;

            for cap in reg.captures_iter(&line) {
                self.verbose(
                    format!("Matched: '{}', extracted substring: {}", &cap[0], &cap[1]),
                    3,
                );
                let matched = &cap[1];

                // handle tilde
                let path = if matched.contains("~") {
                    self.verbose(format!("Found tilde, replacing"), 3);
                    shellexpand::tilde(&matched).to_string()
                } else {
                    matched.to_string()
                };

                let mut context = origin.parent().unwrap().to_path_buf();
                context.push(&path);
                self.verbose(format!("Expanded path: {}", context.display()), 3);

                // absolutize "does not care about whether the file exists and what the file really is",
                // meaning it also returns the absolute path if it does not acutally exists
                // more here: https://crates.io/crates/path-absolutize
                match context.absolutize() {
                    Ok(dest_cow) => {
                        let mut destination = dest_cow.into_owned();
                        destination.push(&origin.file_name().unwrap());
                        self.verbose(format!("Origin: {}", origin.display()), 3);
                        self.verbose(format!("Destination: {}", destination.display()), 3);
                        destinations.push(destination);
                    }
                    Err(err) => self.log(format!("Path {} is not valid: {}", path, err), 3),
                }
            }
        }

        Ok(Link {
            origin: origin.canonicalize()?,
            destinations,
            autolink: self,
        })
    }

    // LOGGING (Printing nicely to console)

    /// Creates a simple spacer consisting of = surrounded by empty lines.
    fn spacer() {
        println!();
        println!("==========================");
        println!();
    }

    /// Log a message with a given indentation-level.
    fn log(&self, message: String, level: usize) {
        let mut prefix = "".to_owned();
        for _i in 0..level {
            prefix += "=";
        }
        prefix += ">";

        println!("{}", format!("{} {}", prefix, message));
    }

    /// Only log a message when verbose is specifed.
    fn verbose(&self, message: String, level: usize) {
        if self.verbose {
            self.log(message, level);
        }
    }
}

// TODO update readme
// TODO create rpm-build and maybe publish to cargo?
// TODO write tests

fn main() -> Result<()> {
    // use human_panic to have a nicer error-message
    setup_panic!();

    // leverages StructOpt to do the CLI-handling and parsing
    let autolink: Autolink = Autolink::from_args();
    autolink.do_op()
}
