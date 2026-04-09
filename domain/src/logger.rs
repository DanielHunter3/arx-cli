use std::io::Write;
use colored::*;
use log::LevelFilter;

fn setup_logger(level: LevelFilter) {
    env_logger::Builder::new()
        .filter_level(level)
        .format(|buf, record| {
            let timestamp = 
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
            
            // Создаем форматированную строку без цвета
            let log_line = format!(
                "{} [{}] {}",
                timestamp,
                record.level(),
                record.args()
            );
            
            // Применяем цвет в зависимости от уровня логирования
            let colored_line = match record.level() {
                log::Level::Error => log_line.red(),
                log::Level::Warn => log_line.yellow(),
                log::Level::Info => log_line.green(),
                log::Level::Debug => log_line.blue(),
                log::Level::Trace => log_line.cyan(),
            };
            
            // Используем write! с форматированием вместо writeln!
            writeln!(buf, "{}", colored_line)
        })
        .init();
}

#[derive(Debug, Clone, Copy)]
pub enum SmartLevel {
    Verbose,
    Debug,
    Release
}

const fn complile_log_level() -> SmartLevel {
    if cfg!(debug_assertions) {
        SmartLevel::Debug
    } else {
        SmartLevel::Release
    }
}

pub fn runtime_log_level(is_verbose: bool) -> SmartLevel {
    if is_verbose {
        return SmartLevel::Verbose;
    } else {
        complile_log_level()
    }
}

pub fn setup_smart_logger(level: SmartLevel) {
    setup_logger(
        match level {
            SmartLevel::Verbose => LevelFilter::Trace,
            SmartLevel::Debug => LevelFilter::Debug,
            SmartLevel::Release => LevelFilter::Info,
        }
    );
}