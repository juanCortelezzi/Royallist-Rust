use royallist::{buffers, icons, paths};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Cli::from_args();
    let stdout = buffers::StdoutBuffer::new();
    if args.files.len() == 1 {
        common_print(&args.files[0], &stdout);
    } else {
        for path in args.files {
            println!("---- {} ----", path.display());
            common_print(&path, &stdout);
            println!()
        }
    }
}

fn common_print(path: &Path, stdout: &buffers::StdoutBuffer) {
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
