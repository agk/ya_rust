use std::fmt::Display;

fn print_pair<T: Display>(a: T, b: T) {
    println!("{}, {}", a, b) ;
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    // Часть 1. Функция print_pair, которая принимает два значения любого типа 
    // и печатает их через запятую. Тип должен поддерживать вывод в консоль.
    print_pair(1, 2);             // → 1, 2
    print_pair("hello", "world"); // → hello, world

    // Часть 2. Функция largest, которая принимает срез значений и возвращает наибольшее. 
    // Тип должен поддерживать сравнение.
    let numbers = vec![34, 50, 25, 100, 65];
    println!("{}", largest(&numbers)); // → 100

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest(&chars));   // → y
}