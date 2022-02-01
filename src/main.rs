#![deny(missing_docs)]

//! https://codereview.stackexchange.com/a/144643

use std::fs::OpenOptions;
use std::io::{self, Read, Write};

use clap::Parser;
use strip_ansi_escapes::Writer as StripAnsiWriter;

#[derive(Parser, Debug)]
#[clap(author, version)]
/// Read from STDIN and write to STDOUT (unchanged) and FILES (without ANSI-colors).
struct Cli {
    /// Output files
    files: Vec<String>,

    /// Append to the given files instead of overwriting
    #[clap(short, long)]
    append: bool,

    /// Strip ANSI color codes when writing to files
    #[clap(long, parse(try_from_str), default_value_t = true)]
    strip_ansi: bool,

    /// Buffer size
    #[clap(long = "bs", default_value_t = 8 * 1024)]
    buffer_size: usize,
}

fn main() {
    let cli = Cli::parse();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let stdout = io::stdout();
    let stdout = stdout.lock();

    let mut outputs: Vec<_> = cli
        .files
        .iter()
        .map(|fname| {
            // Open/create the file
            let f = OpenOptions::new()
                .write(true)
                .create(true)
                .append(cli.append)
                .open(fname)
                .unwrap_or_else(|e| {
                    panic!("Unable to open file '{}': {}", fname, e);
                });
            // Wrap (if needed) the file with StripAnsiWriter
            if cli.strip_ansi {
                Box::new(StripAnsiWriter::new(f)) as Box<dyn Write>
            } else {
                Box::new(f) as Box<dyn Write>
            }
        })
        .collect();

    // Also add STDOUT
    outputs.push(Box::new(stdout));

    // Setup the buffer
    let mut buf = vec![0; cli.buffer_size];

    loop {
        // Read from STDIN
        let bytes = stdin.read(&mut buf).expect("Unable to read from stdin");

        // Exit when STDIN is closed
        if bytes == 0 {
            break;
        }

        // **Very important**, otherwise you can end up with
        // Heartbleed-esque bugs! I'm chosing to shadow `buf` to
        // deliberately prevent using it again in this loop.
        let buf = &buf[..bytes];

        for output in &mut outputs {
            output.write_all(buf).expect("Unable to write to output");
        }
    }
}
