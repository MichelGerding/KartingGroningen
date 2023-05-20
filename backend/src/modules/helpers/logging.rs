use dotenvy::dotenv;
use fern::Dispatch;
use std::env;

pub fn setup_logging() -> Result<(), fern::InitError> {
    dotenv().ok();
    // get log level
    let verbosity = env::var("LOGGING_LEVEL").expect("Failed to get LOGGING_LEVEL from .env file");

    let verbosity_str = verbosity.as_str();

    //TODO:: add logging to discord.
    let mut base_config = Dispatch::new();

    base_config = match verbosity_str {
        "OFF" => base_config.level(log::LevelFilter::Off),
        "ERROR" => base_config.level(log::LevelFilter::Error),
        "WARN" => base_config.level(log::LevelFilter::Warn),
        "DEBUG" => base_config.level(log::LevelFilter::Debug),
        "TRACE" => base_config.level(log::LevelFilter::Trace),
        _ => {
            // default to info
            base_config.level(log::LevelFilter::Info)
        }
    };

    let file_logger_config = Dispatch::new()
        .format(|out, message, record| {
            if record.target() == "handlebars::render" {
                return;
            }
            out.finish(format_args!(
                "{} [{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("../../../program.log").unwrap());

    let stdout_logger_config = Dispatch::new()
        .format(|out, message, record| {
            if record.target().starts_with("rocket") || record.target() == "_" {
                return out.finish(format_args!("{}", message));
            }

            if record.target() == "handlebars::render" {
                return;
            }

            out.finish(format_args!(
                "{} [{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout());

    base_config
        .chain(file_logger_config)
        .chain(stdout_logger_config)
        .apply()?;

    Ok(())
}
