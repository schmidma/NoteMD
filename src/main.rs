use std::{fs::create_dir_all, path::Path, process};

use args::parse_args;
use repository::{clone_repository, sync_repository};

use crate::{logging::setup_logger, tui_select::select_note_with_tui};

mod args;
mod logging;
mod repository;
mod search;
mod tui_select;

fn main() -> anyhow::Result<()> {
    let arguments = parse_args();

    let notes_directory = arguments.value_of("note-directory").unwrap();
    create_dir_all(&notes_directory)?;

    match arguments.subcommand() {
        Some((command, matches)) => {
            setup_logger(arguments.is_present("verbose"))?;
            match command {
                "clone" => clone_repository(notes_directory, matches.value_of("remote").unwrap())?,
                "sync" => sync_repository(notes_directory)?,
                _ => (),
            }
        }
        None => {
            let note_to_open = select_note_with_tui(&notes_directory)?;

            if let Some(file_name) = note_to_open {
                let note_path = Path::new(notes_directory).join(file_name);
                process::Command::new("nvim")
                    .args([note_path])
                    .spawn()?
                    .wait()?;
            };
        }
    };

    Ok(())
}
