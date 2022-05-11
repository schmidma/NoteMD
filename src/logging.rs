use std::path::Path;

pub fn setup_logger<P>(is_verbose: bool, note_directory: P) -> Result<(), fern::InitError>
where
    P: AsRef<Path>,
{
    let level = if is_verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(fern::log_file(note_directory.as_ref().join("notemd.log"))?)
        .apply()?;
    Ok(())
}
