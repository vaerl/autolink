# autolink

autolink is a cli-tool for automatically symlinking files.

Instead of manually going symlinking files, autolink looks for lines containing `##!!`.
If a file contains such a line, autolink will interpret the following characters as a file-path and attempt to symlink the file to the destination.
For example, when parsing `##!!~/dev/test/` in a file called `foo.txt`, the file will be symlinked to `~/dev/test/foo.txt`.

## Features

1. symlink a file or directory: `autolink <path>`
2. symlink a file or directory and create needed dirs: `autolink <path> -c`
3. delete symlinks from a file or directory: `autolink <path to origin> -d`
4. overwrite symlinks: `autolink <path> -o`
   - NOTE: this is not yet tested - use with caution!

## Testing

Run `cargo test`(no tests are implemented yet!).

## Building

Clone the repository, then `cd` into it. Now, run `cargo build --release`.
The binary is now available under `./target/release`.

## Planned features

- [ ] implement tests
- [ ] improve building
  - use gh-actions?
  - write script to copy & replace autolink to sbin(?);

## Examples

TODO
