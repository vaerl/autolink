use anyhow::Result;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use crate::Autolink;

pub struct Link<'a> {
    pub origin: PathBuf,
    pub destinations: Vec<PathBuf>,
    pub(crate) autolink: &'a Autolink,
}

impl<'a> Link<'a> {
    /// Delete the link.
    pub fn delete(&self) -> Result<()> {
        for destination in &self.destinations {
            if destination.exists() {
                self.autolink
                    .log(format!("Removing link {}", destination.display()), 1);
                fs::remove_file(destination)?;
            } else {
                self.autolink
                    .verbose(format!("No link at {}", destination.display()), 1);
            }
        }
        Ok(())
    }

    /// Symlink the link.
    pub fn link(self, overwrite: bool, create_dirs: bool) -> Result<()> {
        for destination in &self.destinations {
            if create_dirs {
                let parent = destination.parent().unwrap();
                self.autolink.log(
                    format!("Creating all directories for path {}", parent.display()),
                    1,
                );
                // destination will always be a file - thus, I need its parent if directories are to be created
                fs::create_dir_all(parent)?;
            }

            if overwrite {
                self.autolink
                    .log(format!("Deleting symlink to overwrite."), 1);
                self.delete()?;
            }

            match symlink(&self.origin, destination) {
                Ok(_res) => self.autolink.log(
                    format!(
                        "Symlinked {} to {}",
                        self.origin.display(),
                        destination.display()
                    ),
                    1,
                ),
                Err(err) => self.autolink.log(
                    format!(
                        "Symlinking {} to {} failed: {}",
                        self.origin.display(),
                        destination.display(),
                        err
                    ),
                    1,
                ),
            }
        }
        Ok(())
    }
}
