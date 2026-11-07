mod math;

use crate::math::basic;

fn main() {
    println!("2 + 3 = {}", basic::add(2, 3));
    println!("7 - 4 = {}", basic::subtract(7, 4));
    println!("6 * 7 = {}", basic::multiply(6, 7));
    println!("20 / 5 = {}", basic::divide(20.0, 5.0));
}