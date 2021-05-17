#[cfg(test)]
mod tests {

    use path_absolutize::Absolutize;

    use crate::{linkfile::LinkFile, Autolink};
    use std::{fs, path::PathBuf};

    // FINDING LINKS

    #[test]
    fn get_links_from_file() {
        let autolink = Autolink {
            path: PathBuf::from("examples/file1.example"),
            create_dirs: false,
            overwrite: false,
            delete: false,
            verbose: false,
        };

        let link_file = autolink
            .get_links(&PathBuf::from("examples/file1.example"))
            .unwrap();
        let expected_destinations = vec![
            PathBuf::from(shellexpand::tilde("~/file1.example").to_string()),
            PathBuf::from(shellexpand::tilde("~/dev/file1.example").to_string()),
            // NOTE I have to pass examples as additional context for the file its being read from
            PathBuf::from(shellexpand::tilde("examples/test/file1.example").to_string())
                .absolutize()
                .unwrap()
                .into_owned(),
        ];
        assert_eq!(link_file.destinations, expected_destinations);
    }

    #[test]
    fn get_linkfiles_from_directory() {
        for res in PathBuf::from("examples/nested-dir").read_dir().unwrap() {
            let dir_entry = res.unwrap();

            if dir_entry.file_name().to_str().unwrap() != "file2.example" {
                fs::remove_file(dir_entry.path()).unwrap();
            }
        }

        assert_eq!(
            PathBuf::from("examples/nested-dir")
                .read_dir()
                .unwrap()
                .count(),
            1
        );

        let autolink = Autolink {
            path: PathBuf::from("examples/nested-dir"),
            create_dirs: false,
            overwrite: false,
            delete: false,
            verbose: false,
        };

        let linkfiles = autolink
            .find_links(&PathBuf::from("examples/nested-dir"))
            .unwrap();
        let expected_linkfiles = vec![LinkFile {
            origin: PathBuf::from("examples/nested-dir/file2.example")
                .absolutize()
                .unwrap()
                .into_owned(),
            autolink: &autolink,
            destinations: vec![PathBuf::from("examples/test/nested-dir/file2.example")
                .absolutize()
                .unwrap()
                .into_owned()],
        }];

        // iterate through both vecs at once
        for (actual, expected) in linkfiles.iter().zip(expected_linkfiles) {
            assert_eq!(actual.origin, expected.origin);
            assert_eq!(actual.destinations, expected.destinations);
        }
    }

    // LINKS

    #[test]
    fn create_link() {
        let autolink = Autolink {
            path: PathBuf::from("examples/file1.example"),
            create_dirs: false,
            overwrite: false,
            delete: false,
            verbose: false,
        };

        let linkfile = LinkFile {
            origin: PathBuf::from("examples/file1.example")
                .absolutize()
                .unwrap()
                .into_owned(),
            destinations: vec![PathBuf::from("examples/nested-dir/file1.example")],
            autolink: &autolink,
        };

        linkfile.link(false, false).unwrap();
        assert!(PathBuf::from("examples/nested-dir/file1.example").is_file());
    }

    #[test]
    fn delete_link() {
        // create the link to not depend on any order of tests
        create_link();

        let autolink = Autolink {
            path: PathBuf::from("examples/file1.example"),
            create_dirs: false,
            overwrite: false,
            delete: false,
            verbose: false,
        };

        let linkfile = LinkFile {
            origin: PathBuf::from("examples/file1.example")
                .absolutize()
                .unwrap()
                .into_owned(),
            destinations: vec![PathBuf::from("examples/nested-dir/file1.example")],
            autolink: &autolink,
        };

        linkfile.delete().unwrap();
        assert!(!PathBuf::from("examples/nested-dir/file1.example").is_file());
    }
}
