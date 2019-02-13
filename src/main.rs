#[macro_use]
extern crate slog;
#[macro_use]
extern crate clap;
extern crate slog_bunyan;

use std::sync::Mutex;

use slog::{Drain, Level, LevelFilter, Logger};

mod opts;

fn main() {
    let matches = opts::parse("logging-test".to_string());

    let level = matches.value_of("level").unwrap_or("info");

    println!("want level: {:?}", level);

    let filter_level = match level.parse::<Level>() {
        Ok(filter_level) => {
            println!("parsed level: {:?}", filter_level);
            filter_level
        }
        Err(err) => {
            println!("failed to match type {:?}", err);
            std::process::exit(1);
        }
    };

    let stream = slog_bunyan::default(std::io::stdout());
    let log = Logger::root(
        Mutex::new(LevelFilter::new(stream, filter_level)).fuse(),
        o!("name" => "logging-test"),
    );

    trace!(log, "trace message");
    info!(log, "that's a paddlin'");
    crit!(log, "X_X");
}
