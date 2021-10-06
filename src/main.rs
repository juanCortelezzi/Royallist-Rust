use royallist::path;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use termcolor::{Buffer, BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

/// read_dir reads the passed directory and returns a vec with its contents
fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}

fn write_entry(stdout: &mut Buffer, icon: char, name: &str, color: Color) -> io::Result<()> {
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(stdout, "{} ", icon)?;
    stdout.reset()?;
    writeln!(stdout, "{}", name)?;
    Ok(())
}

fn main() {
    let args = Cli::from_args();
    let mut entries = read_dir(&args.path).expect("could not read dir");

    entries.sort();

    let bufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut buffer = bufwtr.buffer();

    let dirbufwtr = BufferWriter::stderr(ColorChoice::Always);
    let mut dirbuffer = bufwtr.buffer();

    for entry in entries {
        let name = path::path_filename(&entry);

        if entry.as_path().is_dir() {
            write_entry(&mut dirbuffer, '', &name, Color::Cyan).unwrap();
        } else {
            let (icon, color) = path::match_icon(&path::path_extension(&entry));
            write_entry(&mut buffer, icon, &name, color).unwrap();
        }
    }

    dirbufwtr.print(&dirbuffer).unwrap();
    bufwtr.print(&buffer).unwrap();
}
