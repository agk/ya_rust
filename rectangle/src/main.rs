struct Rectangle {
    width: f64,
    height: f64,
}

fn area(rect: Rectangle) -> f64 {
    rect.width * rect.height
}

fn perimeter(rect: Rectangle) -> f64 {
    2.0 * (rect.width + rect.height)
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn swap_tuple(pair: (i32, i32)) -> (i32, i32) {
    (pair.1, pair.0)
}

fn main() {
    println!("Площадь: {}", area(Rectangle { width: 25.0, height: 2.0 }));
    println!("Периметр: {}", perimeter(Rectangle { width: 25.0, height: 2.0 }));
    println!("22°C = {}", celsius_to_fahrenheit(22.0));
    println!("71.6°F = {}", fahrenheit_to_celsius(71.6));
    println!("swap = {:?}", swap_tuple((1,2)));
}
