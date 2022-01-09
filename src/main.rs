mod buffers;
mod icons;
mod paths;
mod prints;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(default_value = ".", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let args = Cli::from_args();
    let stdout = buffers::StdoutBuffer::new();
    if args.files.len() == 1 {
        prints::common_print(&args.files[0], &stdout);
    } else {
        for path in args.files {
            println!("---- {} ----", path.display());
            prints::common_print(&path, &stdout);
            println!()
        }
    }
}
