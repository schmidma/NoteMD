use std::{env, fs::create_dir_all, path::Path, process};

use anyhow::Context;
use args::parse_args;
use repository::{clone_repository, commit_changes, is_note_changed, sync_repository};

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
                let editor = env::var("EDITOR").context("Could not determine default editor")?;
                process::Command::new(editor)
                    .arg(&note_path)
                    .spawn()?
                    .wait()?;
                if is_note_changed(notes_directory, &note_path)? {
                    commit_changes(notes_directory, &note_path)?;
                }
            };
        }
    };

    Ok(())
}
