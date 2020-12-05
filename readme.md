# autolink

autolink is a cli-tool for automatically symlinking files.

Instead of manually going symlinking files, autolink looks for lines containing `##!!`.
If a file contains such a line, autolink will interpret the following characters as a file-path and attempt to symlink the file to the destination.
For example, when parsing `##!!~/dev/test/` in a file called `foo.txt`, the file will be symlinked to `~/dev/test/foo.txt`.

## Features

- symlink files: `autolink ./examples/file.example`
- symlink directories recursively: `autolink ./example`

## Testing

Run `cargo test`(no tests are implemented yet!).

## Building

Clone the repository, then `cd` into it. Now, run `cargo build --release`.
The binary is now available under `./target/release`.

## Planned features

- [ ] save linked files for overview/deleting
- [ ] delete created symlinks
- [ ] allow overriding -> aks for user-input or pass option-tag
- [ ] implement tests
- [ ] improve building

## Examples

TODO
