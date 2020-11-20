use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

// using structopt auto-generates CLI-information
#[derive(StructOpt)]
struct Arg {
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() -> Result<()> {
    // TODO add logging

    // read arguments into arg - is the struct necessary?
    let arg = Arg::from_args();
    if arg.path.is_file() {
        link(arg.path)
    } else if arg.path.is_dir() {
        // TODO get all files, iterate over
        link(arg.path)
    } else {
        Ok(())
    }
}

fn link(origin: PathBuf) -> Result<()> {
    // open file from path
    let content: String = std::fs::read_to_string(&origin).with_context(|| {
        // format!(
        //     "Could not read file `{}`",
        //     &arg.path.into_os_string().into_string().unwrap() // this works - but is it nice?
        // )
        format!("Could not read file")
    })?;

    for line in content.lines() {
        if line.contains("##!!") {
            // TODO optimize -> use bufReader?
            println!("{}", line);

            // cut indicator from line, convert to path
            let substr: String = line.chars().into_iter().skip(4).collect();
            // TODO check what happens when the substr is invalid!
            let destination: PathBuf = PathBuf::from(substr);
            // TODO check if symlink exists
            // symlink from the given path to destination
            // TODO convert to complete paths
            // TODO is the & needed at origin?
            Command::new("ln")
                .arg("-s")
                .arg(&origin)
                .arg(destination)
                .spawn()
                .expect("Could not symlink.");
        }
    }
    Ok(())
}
