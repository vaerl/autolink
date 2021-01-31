# autolink

autolink is a cli-tool for automatically symlinking files.

Instead of manually symlinking files, autolink looks for lines containing `##!!`.
Autolink will then try to read the following characters as a path and attempt to symlink the file to the destination.

For example, when parsing `##!!~/dev/test/` in a file called `foo.txt`, the file will be symlinked to `~/dev/test/foo.txt`.

## Features

1. symlink a file or directory: `autolink <path>`
2. delete symlinks specified a file or directory: `autolink <path to origin> -d`

## Planned Features

1. symlink a file or directory and create needed dirs: `autolink <path> -c`
2. overwrite symlinks: `autolink <path> -o`

## Testing

Run `cargo test`(no tests are implemented yet!).

## Building

Clone the repository, then `cd` into it. Now, run `cargo build --release`.
The binary is now available under `./target/release`.

## Examples

TODO
