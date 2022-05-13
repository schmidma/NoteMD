use clap::{arg, command, ArgMatches};
use home::home_dir;

fn get_default_notes_directory() -> String {
    home_dir()
        .expect("Unable to find home directory")
        .join(".notes")
        .to_str()
        .expect("Unable to decode Path to notes directory")
        .to_string()
}

pub fn parse_args() -> ArgMatches {
    command!()
        .arg(arg!(-v --verbose "use verbose logging"))
        .arg(
            arg!(
                --"note-directory" <DIR> "Directory to read and write notes"
            )
            .required(false)
            .default_value(&get_default_notes_directory()),
        )
        .subcommand(
            clap::Command::new("clone")
                .about("Clones a remote repository to the notes directory")
                .arg(arg!([remote] "Remote repository url to clone")),
        )
        .subcommand(clap::Command::new("sync").about("Synchronize (pull/push) notes repository"))
        .get_matches()
}
