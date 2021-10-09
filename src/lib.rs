pub mod buffers;
pub mod icons;
pub mod paths;

pub mod prints {
    use crate::icons;
    use crate::paths;
    use std::path::Path;

    pub fn common_print(path: &Path, stdout: &crate::buffers::StdoutBuffer) {
        let mut entries = paths::read_dir(&path).expect("could not read dir");

        entries.sort();

        let mut filebuf = stdout.buffer();
        let mut dirbuf = stdout.buffer();

        for entry in entries {
            let name = paths::get_filename(&entry);

            if entry.as_path().is_dir() {
                let icon = icons::match_dir_icon(&name);
                stdout.write_entry(&mut dirbuf, icon, &name).unwrap();
            } else {
                let icon = icons::match_ext_icon(&paths::get_extension(&entry));
                stdout.write_entry(&mut filebuf, icon, &name).unwrap();
            }
        }

        stdout.flush(&dirbuf).unwrap();
        stdout.flush(&filebuf).unwrap();
    }
}
