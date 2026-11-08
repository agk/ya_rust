use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let random_number = rng.random_range(1..=100);
    
    println!("Случайное число от 1 до 100: {random_number}");
}
