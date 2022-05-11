use std::fs::create_dir_all;

use clap::{arg, command};
use home::home_dir;

use crate::logging::setup_logger;

mod logging;

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
    Ok(())
}
