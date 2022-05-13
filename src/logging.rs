use std::io::stdout;

use fern::Dispatch;

fn base_config(is_verbose: bool) -> Dispatch {
    let level = if is_verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    fern::Dispatch::new().level(level)
}

fn stdout_logging() -> Dispatch {
    fern::Dispatch::new()
        .format(|out, message, record| out.finish(format_args!("[{}] {}", record.level(), message)))
        .chain(stdout())
}

pub fn setup_logger(is_verbose: bool) -> Result<(), log::SetLoggerError> {
    let base_config = base_config(is_verbose);
    base_config.chain(stdout_logging()).apply()
}
