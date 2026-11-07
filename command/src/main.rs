enum Command {
    Show(&'static str),
    Add(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
    Stop,
}

fn calculate(command: &Command) -> Option<i32> {
    match command {
        Command::Add(a, b) => Some(a + b),
        Command::Multiply(a, b) => Some(a * b),
        Command::Divide(_, b) if *b == 0 => None, 
        Command::Divide(a, b) => Some(a / b), 
        Command::Show(_) | Command::Stop => None, 
    }
}

fn main() {
    let commands = vec!(
        Command::Show("Стартуем"), 
        Command::Add(2, 3),
        Command::Divide(10, 0),
        Command::Multiply(4, 5),
        Command::Show(""),
        Command::Show("Финишная прямая"),
        Command::Stop, 
        Command::Add(100, 1)
    );    

    for command in &commands {
        match command {
            Command::Show(text) if text.is_empty() => continue,
            Command::Show(text) => {
                if let Some(fist) = text.chars().next() {
                    println!("Сообщение: {text}. Первый символ: {fist}")
                }
                continue;
            }
            Command::Stop => {
                println!("Останавливаем обработку команд.");
                break;
            }
            _ => {}

        }

        match calculate(command) {
            Some(value) => println!("Результат: {value}"),
            None => println!("Операция не выполнена"),
        }
    }
}
