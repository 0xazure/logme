use chrono::Local;
use clap::{crate_name, crate_version, App, Arg, SubCommand};
use dirs;
use std::convert::Into;
use std::env;
use std::error;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

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

fn start_editor_child_process<F: AsRef<OsStr>>(
    editor: Option<OsString>,
    filepath: &F,
) -> io::Result<ExitStatus> {
    let editors = [
        editor,
        Some(OsString::from("vim")),
        Some(OsString::from("vi")), // Some distributions still call it `vi`
    ];

    let mut editor_child_process = Err(io::Error::from(io::ErrorKind::NotFound));

    // Try each editor in the list, using the first one that successfully spawns as a child process
    for maybe_editor in &editors {
        if let Some(editor) = maybe_editor {
            let result = Command::new(&editor).arg(filepath).spawn();
            if result.is_ok() {
                editor_child_process = result;
                break;
            }
        }
    }

    editor_child_process
        .expect("Failed to start editor. Ensure $EDITOR is set or vim is installed.")
        .wait()
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .arg(Arg::with_name("message").help("Message to record").index(1))
        .subcommand(SubCommand::with_name("edit").about("Manually edit today's message log"))
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

    match matches.subcommand() {
        ("", None) => {
            if let Some(message) = matches.value_of("message") {
                if !message.is_empty() {
                    writeln!(&file, "{}", message)?;
                }
            } else {
                read_lines(&file);
            }
        }
        ("edit", Some(_)) => {
            start_editor_child_process(env::var_os("EDITOR"), &filepath)?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
