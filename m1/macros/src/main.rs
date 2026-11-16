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

fn main() {
    say_hello!();
    say!("Привет, мир!");
    say!("Сегодня мы учим макросы в Rust");
}