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

### Locally

Install [Rust](https://www.rust-lang.org/) if you haven't already.
Then run the build- and install-scripts from [here](./scripts).

### GitHub

On a push to origin with a tag containing the version number, a GitHub-Actions workflow is triggered that builds a linux binary.
Creating a new release:

Use `git tag vx.x.x` for a minor or `git tag -a vx.x.x -m "message"` for a major release (with notes about the release).
Then, run `push origin vx.x.x`.

## Examples

TODO
