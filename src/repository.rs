use std::{path::Path, process::Command};

use log::info;

pub fn clone_repository<P>(notes_directory: P, remote: &str) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    info!(
        "Cloning from '{remote}' to notes directory {}",
        notes_directory.as_ref().display()
    );
    Command::new("git")
        .args(["clone", remote])
        .arg(notes_directory.as_ref())
        .spawn()?
        .wait()?;
    Ok(())
}

pub fn sync_repository<P>(notes_directory: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    info!("Synchronizing notes repository");
    Command::new("git")
        .arg("-C")
        .arg(notes_directory.as_ref())
        .args(["pull", "--rebase"])
        .spawn()?
        .wait()?;
    Command::new("git")
        .arg("-C")
        .arg(notes_directory.as_ref())
        .args(["push"])
        .spawn()?
        .wait()?;
    Ok(())
}

pub fn is_note_changed<D, N>(notes_directory: D, note: N) -> anyhow::Result<bool>
where
    D: AsRef<Path>,
    N: AsRef<Path>,
{
    let exit_status = Command::new("git")
        .arg("-C")
        .arg(notes_directory.as_ref())
        .arg("diff")
        .arg("--exit-code")
        .arg(note.as_ref())
        .spawn()?
        .wait()?;
    Ok(exit_status.code().unwrap() == 1)
}

pub fn commit_changes<D, N>(notes_directory: D, note: N) -> anyhow::Result<()>
where
    D: AsRef<Path>,
    N: AsRef<Path>,
{
    info!("Committing changes on {}", note.as_ref().display());
    Command::new("git")
        .arg("-C")
        .arg(notes_directory.as_ref())
        .arg("add")
        .arg(note.as_ref())
        .spawn()?
        .wait()?;
    Command::new("git")
        .arg("-C")
        .arg(notes_directory.as_ref())
        .arg("commit")
        .arg("-m")
        .arg(format!(
            "Change: {}",
            note.as_ref().file_stem().unwrap().to_str().unwrap()
        ))
        .spawn()?
        .wait()?;
    Ok(())
}
