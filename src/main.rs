mod errors;
mod file_buffer;
mod file_view;
mod ui;
mod utils;

use crate::file_view::BufferedFileView;
use clap::Parser;
use gag::Redirect;
use std::{fs, io};

#[derive(Parser)]
struct Args {
    /// Path to the file to read
    path: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let _redirect_stderr = if atty::is(atty::Stream::Stderr) {
        Some(
            Redirect::stderr(
                fs::OpenOptions::new()
                    .truncate(true)
                    .read(true)
                    .create(true)
                    .write(true)
                    .open("/dev/null")
                    .unwrap(),
            )
            .unwrap(),
        )
    } else {
        None
    };
    return match BufferedFileView::new(args.path) {
        Ok(file_view) => ui::run(Box::new(file_view)),
        Err(err) => Err(err),
    };
}
