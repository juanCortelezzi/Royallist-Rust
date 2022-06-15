use std::io::{self, Write};
use termcolor::{Buffer, BufferWriter, ColorChoice, ColorSpec, WriteColor};

use crate::icons::Icon;

pub struct StdoutBuffer {
    bufwtr: BufferWriter,
}

impl StdoutBuffer {
    pub fn new() -> Self {
        Self {
            bufwtr: BufferWriter::stdout(ColorChoice::Always),
        }
    }

    pub fn flush(&self, buffer: &Buffer) -> io::Result<()> {
        self.bufwtr.print(buffer)
    }

    pub fn buffer(&self) -> Buffer {
        self.bufwtr.buffer()
    }

    pub fn write_entry(&self, buffer: &mut Buffer, icon: Icon, name: &str) -> io::Result<()> {
        buffer.set_color(ColorSpec::new().set_fg(Some(icon.1)))?;
        write!(buffer, "{} ", icon.0)?;
        buffer.reset()?;
        writeln!(buffer, "{}", name)
    }
}

impl Default for StdoutBuffer {
    fn default() -> Self {
        StdoutBuffer::new()
    }
}
