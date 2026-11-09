fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply_fn(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn apply_impl(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn apply_dyn(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn apply_once<T, U, F>(value: T, func: F) -> U
where
    F: FnOnce(T) -> U,
{
    func(value)
}

fn create_counter(start: i32) -> impl FnMut() -> i32 {
    let mut count = start;
    move || {
        count += 1;
        count
    }
}

fn greeter(name: String) {
    let hello = || println!("Hello, {}!", name);
    let bye = || println!("Bye, {}!", name);
    hello();
    bye();
}

fn main() {
    // 7. вызовите apply_fn(add_one, 5);
    println!("apply_fn: {}", apply_fn(add_one, 5));
    
    // 8. создайте non-capturing closure и положите его в переменную типа fn(i32) -> i32;
    let as_fn: fn(i32) -> i32 = |x| x * 2;
    println!("fn closure: {}", apply_fn(as_fn, 5));
    
    // 9. создайте capturing closure с offset = 10 и передайте его в apply_impl и apply_dyn;
    let offset = 10;
    let add_offset = |x| x + offset;   // capturing closure: захватывает offset по &
    // Статическая диспетчеризация (impl Fn)
    println!("{}", apply_impl(add_offset, 5));   // 15
    // Динамическая диспетчеризация (&dyn Fn)
    println!("{}", apply_dyn(&add_offset, 7));   // 17

    // 10. отдельно покажите apply_once на значении String, чтобы было видно, 
    // что замыкание может забрать владение аргументом;
    let len = apply_once(String::from("привет"), |text| text.len());
    println!("apply_once len длина: {len}");

    // 11. создайте счётчик через create_counter(0) и вызовите его два раза;
    let mut counter = create_counter(0);
    println!("{}", counter());   // 1
    println!("{}", counter());   // 2

    // 12. отдельно покажите пример move с i32, 
    // после которого исходная переменная всё ещё доступна;
    let number = 7;
    let moved_copy = move || number + 1;
    println!("moved copy result: {}", moved_copy());
    println!("number still here: {}", number);

    // 13. вызовите greeter(String::from("Rust"))
    greeter(String::from("Rust"));

}