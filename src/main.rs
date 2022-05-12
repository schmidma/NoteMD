use std::{fs::create_dir_all, path::Path, process::Command};

use clap::{arg, command};
use home::home_dir;

use crate::{logging::setup_logger, tui_select::select_note_with_tui};

mod logging;
mod search;
mod tui_select;

fn get_default_notes_directory() -> String {
    home_dir()
        .expect("Unable to find home directory")
        .join(".notes")
        .to_str()
        .expect("Unable to decode Path to notes directory")
        .to_string()
}

fn main() -> anyhow::Result<()> {
    let matches = command!()
        .arg(arg!(-v --verbose "use verbose logging"))
        .arg(
            arg!(
                --"note-directory" <DIR> "Directory to read and write notes"
            )
            .required(false)
            .default_value(&get_default_notes_directory()),
        )
        .get_matches();

    let note_directory = matches.value_of("note-directory").unwrap();
    create_dir_all(&note_directory)?;
    setup_logger(matches.is_present("verbose"), &note_directory)?;

    let note_to_open = select_note_with_tui(&note_directory)?;

    if let Some(file_name) = note_to_open {
        let note_path = Path::new(note_directory).join(file_name);
        Command::new("nvim").args([note_path]).spawn()?.wait()?;
    };

    Ok(())
}
