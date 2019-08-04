use chrono::Local;
use clap::{crate_name, crate_version, App, Arg};
use dirs;
use std::convert::Into;
use std::env;
use std::error;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

fn xdg_data_home() -> PathBuf {
    dirs::home_dir()
        .expect("$HOME must be set")
        .join(format!(".local/share/{}", env!("CARGO_PKG_NAME")))
}

fn read_lines(file: &File) {
    let buff = BufReader::new(file);
    buff.lines()
        .map(Result::unwrap_or_default)
        .filter(|l| !l.is_empty())
        .for_each(|l| println!("- {}", l));
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .arg(Arg::with_name("message").help("Message to record").index(1))
        .get_matches();

    // Get or compute XDG_DATA_HOME path
    let data_dir = env::var_os("XDG_DATA_HOME").map_or_else(xdg_data_home, Into::into);

    // Ensure XDG_DATA_HOME path exists
    fs::create_dir_all(&data_dir)?;

    // Create file path as an ISO 8601-formatted date
    let filepath = data_dir.join(format!("{}.txt", Local::now().date().format("%Y-%m-%d")));

    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(&filepath)?;

    if let Some(message) = matches.value_of("message") {
        if !message.is_empty() {
            writeln!(&file, "{}", message)?;
        }
    } else {
        read_lines(&file);
    }

    Ok(())
}
