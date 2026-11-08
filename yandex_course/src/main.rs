// fn longest_word<'a>(words: &[&'a str]) -> Option<&'a str> {
//     words.iter().copied().max_by_key(|w| w.len())
// }

fn longest_word<'a>(words: &[&'a str]) -> Option<&'a str> {
    let mut best: Option<&'a str> = None;
    for &w in words {                 // &w разыменовывает &&'a str → &'a str
        match best {
            Some(b) if b.len() >= w.len() => {}
            _ => best = Some(w),
        }
    }
    best
}

fn main() {
    let words = ["rust", "ownership", "borrow"];
    let result = longest_word(&words);
    println!("{result:?}"); // Some("ownership")
}