# autolink

autolink is a CLI-tool for automatically symlinking files: 
it looks for lines containing `##!!` and then tries to read the following characters as a path and to symlinks the file to the specified destination.

For example, when parsing `##!!~/dev/test/` in a file called `foo.txt`, the file will be symlinked to `~/dev/test/foo.txt`.
autolink also supports linking a file to multiple destinations.

## Features

1. create symlinks specifed in a file or directory: `autolink <path>`
    - **this will fail if needed directories are not present!**
2. create symlinks and needed directories: `autolink -c <path>`
3. delete symlinks specified in a file or directory: `autolink -d <path>`
4. overwrite symlinks: `autolink -o <path>`

To see verbose output, use `autolink -v <path>`.

## Testing

Clone the code and run `cargo test`.

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

1. symlink a file: `autolink examples/file1.example`
2. symlink a file and create needed directories: `autolink -c examples/file1.example`
3. symlink a directory recursively: `autolink examples/`
4. delete all symlinks: `autolink -d examples/`
5. overwrite (deletes and recreates) all symlinks: `autolink -o examples/`
