#[macro_export]
macro_rules! say_hello {
     () => {
        println!("Hello, world!");
    };
}

#[macro_export]
macro_rules! say {
    ($msg:expr) => {
        println!("{}", $msg);
    };
}

macro_rules! log {
    ($level:expr, $msg:expr) => { 
        println!("[{}] {}", $level, $msg);
    }
}

fn main() {
    say_hello!();
    say!("Привет, мир!");
    say!("Сегодня мы учим макросы в Rust");

    log!("INFO", "Сервер запущен");
    log!("ERROR", "Соединение потеряно");
}