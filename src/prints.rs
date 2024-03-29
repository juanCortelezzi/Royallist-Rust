use crate::icons;
use crate::paths;
use std::path::Path;

pub fn common_print(path: &Path, stdout: &crate::buffers::StdoutBuffer) {
    let mut entries = paths::read_dir(path).expect("could not read dir");

    entries.sort();

    let mut filebuf = stdout.buffer();
    let mut dirbuf = stdout.buffer();

    for entry in entries {
        let name = paths::get_filename(&entry);

        if entry.as_path().is_dir() {
            let icon = icons::match_dir_icon(&name)
                .unwrap_or_else(|| icons::get_default_icon(icons::PathType::Dir));

            stdout.write_entry(&mut dirbuf, icon, &name).unwrap();
            continue;
        }

        let extension = paths::get_extension(&entry);

        if let Some(icon) = icons::match_ext_icon(&extension) {
            stdout.write_entry(&mut filebuf, icon, &name).unwrap();
            continue;
        }

        let icon = icons::match_name_icon(&name)
            .unwrap_or_else(|| icons::get_default_icon(icons::PathType::File));

        stdout.write_entry(&mut filebuf, icon, &name).unwrap();
    }

    stdout.flush(&dirbuf).unwrap();
    stdout.flush(&filebuf).unwrap();
}
