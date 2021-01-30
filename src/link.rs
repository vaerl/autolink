use anyhow::Result;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

pub struct Link {
    pub origin: PathBuf,
    pub destinations: Vec<PathBuf>,
}

impl Link {
    pub fn delete(&self) -> Result<()> {
        for destination in &self.destinations {
            if destination.exists() {
                println!("Removing link '{}'.", destination.display());
                fs::remove_file(destination)?;
            }
        }
        Ok(())
    }

    pub fn link(self, overwrite: bool, create_dirs: bool) -> Result<()> {
        for destination in &self.destinations {
            if create_dirs {
                println!(
                    "Creating all directories for path '{}'.",
                    destination.parent().unwrap().display()
                );
                // destination will always be a file - thus, I need its parent if directories are to be created
                fs::create_dir_all(destination.parent().unwrap())?;
            }

            if overwrite {
                println!("Deleting symlink to overwrite.");
                self.delete()?;
            }

            match symlink(&self.origin, destination) {
                Ok(_res) => println!(
                    "Symlinked '{}' to '{}'.",
                    self.origin.display(),
                    destination.display()
                ),
                Err(err) => {
                    println!(
                        "Symlinking of '{}' to '{}' failed with: {}",
                        self.origin.display(),
                        destination.display(),
                        err
                    )
                }
            }
        }
        Ok(())
    }
}
