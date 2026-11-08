use std::fmt;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangle(width={}, height={})", self.width, self.height)
    }
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn main() {
    let rec = Rectangle { width: 10, height: 5 };

    // 1. Через {} — Display
    println!("{}", rec);

    // 2. Через {:?} — Debug (derive)
    println!("{:?}", rec);

    // 3. Через {:#?} — Debug pretty-print
    println!("{:#?}", rec);

    // 4. Строка-отчёт через format!
    let report = format!("{} | area={}", rec, rec.area());
    println!("{report}");

    // 5. Пример выравнивания ширины через {:>4}
    println!("{:>4}", 42);
    println!("{:>4}", 7);
    println!("{:>4}", 123);
}
