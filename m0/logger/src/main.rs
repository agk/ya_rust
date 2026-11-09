use std::path::Path;
use std::io::Write;

struct LoggerConfig {
    level: String,
    file_path: String,
}

#[derive(Debug)]
enum LoggerError {
    ReadConfig(std::io::Error),
    OpenLog(std::io::Error),
    MissingField(&'static str),
    InvalidLevel(String),
    EmptyPath,
}

impl std::fmt::Display for LoggerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoggerError::ReadConfig(_) => write!(f, "не удалось прочитать конфиг"),
            LoggerError::OpenLog(e) => write!(f, "open log: {e}"),
            LoggerError::MissingField(name) => write!(f, "missing field: {name}"),
            LoggerError::InvalidLevel(l) => write!(f, "invalid level: {l}"),
            LoggerError::EmptyPath => write!(f, "empty path"),
        }
    }
}

// Реализация трейта Error
impl std::error::Error for LoggerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LoggerError::ReadConfig(err) => Some(err),   // io::Error — есть Error
            LoggerError::OpenLog(err) => Some(err),      // io::Error — есть Error
            LoggerError::MissingField(_) => None,        // строка — нет Error
            LoggerError::InvalidLevel(_) => None,        // строка — нет Error
            LoggerError::EmptyPath => None,
        }
    }
}
  
fn find_value<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    for line in text.lines() {
        let Some((k, v)) = line.split_once('=') else { continue; };
        if k.trim() == key {
            return Some(v.trim());
        }
    }
    None
}

fn parse_config(text: &str) -> Result<LoggerConfig, LoggerError> {
    // 1. Достаём поля через ok_or_else (None → ошибка)
    let level_str = find_value(text, "level")
        .ok_or_else(|| LoggerError::MissingField("level"))?;
    let file = find_value(text, "file")
        .ok_or_else(|| LoggerError::MissingField("file"))?; 
    
    // 2. Проверяем путь не пустой
    if file.trim().is_empty() {
        return Err(LoggerError::EmptyPath);
    }

    // 3. Парсим уровень через and_then-цепочку
    let level = parse_level(level_str)
        .ok_or_else(|| LoggerError::InvalidLevel(level_str.to_string()))?;

    Ok(LoggerConfig { level, file_path: file.to_string() })
}

fn init_logger(path: &Path) -> Result<LoggerConfig, LoggerError> {
    let text = std::fs::read_to_string(path)
        .map_err(LoggerError::ReadConfig)?;

    let config = parse_config(&text)?;

    let mut log = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.file_path)
        .map_err(LoggerError::OpenLog)?;

    writeln!(log, "logger initialized").map_err(LoggerError::OpenLog)?;    
    
    Ok(config)
}

fn parse_level(s: &str) -> Option<String> {
    match s.to_lowercase().as_str() {
        "trace" | "debug" | "info" | "warn" | "error" => Some(s.to_lowercase()),
        _ => None,
    }
}

fn main() {
    // 1. Успешный
    let valid = "level=info\nfile=/tmp/rust-course-app.log";
    let cfg_path = "/tmp/rust-course-config.txt";
    std::fs::write(cfg_path, valid).expect("write valid config");

    match init_logger(Path::new(cfg_path)) {
        Ok(cfg) => println!("logger ready: {} -> {}", cfg.level, cfg.file_path),
        Err(e) => println!("unexpected error: {e}"),
    }

    // 2. Нет конфига
    let missing = "/tmp/rust-course-missing-config.txt";
    let _ = std::fs::remove_file(missing);
    match init_logger(Path::new(missing)) {
        Ok(_) => println!("unexpected: config found"),
        Err(_) => println!("missing config handled"),
    }

    // 3. Лог в несуществующую директорию
    let bad_dir = "/tmp/rust-course-no-such-dir";
    let _ = std::fs::remove_dir_all(bad_dir);
    let bad_cfg = format!("level=debug\nfile={bad_dir}/app.log");
    let bad_path = "/tmp/rust-course-bad-config.txt";
    std::fs::write(bad_path, bad_cfg).expect("write bad config");

    match init_logger(Path::new(bad_path)) {
        Ok(_) => println!("unexpected: log opened"),
        Err(_) => println!("write error handled"),
    }

}