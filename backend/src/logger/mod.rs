use crate::config::CONFIG;

pub fn init() {
    std::env::set_var("RUST_LOG", CONFIG.app.log_level.as_str());

    pretty_env_logger::env_logger::builder()
        .format_timestamp_millis()
        .init();

    #[cfg(all(unix, not(unix)))]
    {
        use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn")),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    }
}

#[cfg(all(unix, not(unix)))]
mod unused {
    use std::{env, fs, path::Path};

    use log::{debug, LevelFilter};
    use log4rs::{
        append::{
            console::{ConsoleAppender, Target},
            file::FileAppender,
        },
        config::{Appender, Config, Logger, Root},
        filter::threshold::ThresholdFilter,
    };

    pub fn init2() {
        let program_name = get_program_name();
        let mut tmp_file = program_name;
        if cfg!(target_os = "windows") {
            tmp_file = format!("{tmp_file}.txt");
        }
        let mut tmp_file = env::temp_dir().join(tmp_file);
        tmp_file.shrink_to_fit();
        fs::create_dir_all(tmp_file.parent().unwrap()).unwrap();

        let stdout = ConsoleAppender::builder().target(Target::Stdout).build();
        let log = FileAppender::builder().build(&tmp_file).unwrap();

        let stdout_threshold = if cfg!(debug_assertions) {
            LevelFilter::Trace
        } else {
            LevelFilter::Info
        };

        let config = Config::builder();
        let config = config.appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Debug)))
                .build("logfile", Box::new(log)),
        );
        let config = config.appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(stdout_threshold)))
                .build("stdout", Box::new(stdout)),
        );
        let config = config
            .logger(Logger::builder().build("mio", LevelFilter::Error))
            .logger(Logger::builder().build("html5ever", LevelFilter::Error))
            .logger(Logger::builder().build("rustls", LevelFilter::Error))
            .logger(Logger::builder().build("selectors", LevelFilter::Error))
            .logger(Logger::builder().build("want", LevelFilter::Error))
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stdout")
                    .build(LevelFilter::Trace),
            )
            .unwrap();

        log4rs::init_config(config).unwrap();

        debug!("Logging to {:?}", &tmp_file);
    }

    fn get_program_name() -> String {
        let args = env::args().collect::<Vec<String>>();
        let program_name = Path::new(args.first().unwrap())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        program_name.to_string()
    }
}
