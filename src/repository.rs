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
