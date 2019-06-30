use chrono::Local;
use dirs;
use std::convert::Into;
use std::env::{self, Args};
use std::error;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

fn xdg_data_home() -> PathBuf {
    dirs::home_dir()
        .expect("$HOME must be set")
        .join(format!(".local/share/{}", env!("CARGO_PKG_NAME")))
}

fn write_line(args: Args, mut file: File) -> io::Result<()> {
    // Collect all program "arguments" as the message to store separated by spaces
    let note = args.collect::<Vec<_>>().join(" ");
    writeln!(file, "{}", note)
}

fn read_lines(file: &File) {
    let buff = BufReader::new(file);
    buff.lines()
        .map(Result::unwrap_or_default)
        .filter(|l| !l.is_empty())
        .for_each(|l| println!("{}", l));
}

fn main() -> Result<(), Box<dyn error::Error>> {
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

    let mut args = env::args();
    args.next(); // Skip args[0], which is (usually) the program name

    match args.len() {
        0 => read_lines(&file),
        _ => write_line(args, file)?,
    };

    Ok(())
}
