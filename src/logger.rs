use chrono::Local;
use colored::*;
use fern::Dispatch;
use log::{Level, LevelFilter};

fn level_color(level: Level) -> Color {
    match level {
        Level::Trace => Color::Magenta,
        Level::Debug => Color::Cyan,
        Level::Info => Color::Green,
        Level::Warn => Color::Yellow,
        Level::Error => Color::Red,
    }
}

pub fn init_logger(log_level: LevelFilter) {
    Dispatch::new()
        .format(|out, message, record| {
            let color = level_color(record.level());
            out.finish(format_args!(
                "{}: {}",
                format!(
                    "[{}] [{}]",
                    Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level()
                )
                .color(color),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
