#[derive(Default)]
struct FeedOptions {
    compact: bool,
    limit: u8,
}

struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self{ first, second }
    }

    fn first(&self) -> &T {
        &self.first
    }

    fn second(&self) -> &T {
        &self.second
    }
}

impl Pair<f64> {
    fn average(&self) -> f64 {
        (&self.first + &self.second) / 2.0    
    }
}

trait Summary {
    const KIND: &'static str;

    fn summary(&self) -> String;

    fn headline(&self) -> String {
        format!("[{}] {}", Self::KIND, self.summary())
    }
}

struct Article {
    title: String,
    author: String,
}

struct Note {
    text: String,
}

impl Summary for Article {
    const KIND: &'static str = "Article";

    fn summary(&self) -> String {
        format!("{} ({})", self.title, self.author)
    }
}

impl Summary for Note {
    const KIND: &'static str = "Note";

    fn summary(&self) -> String {
        self.text.clone()
    }
}

fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.headline());
}

fn main() {
    let grades = Pair::new(10, 20);
    println!("Первая оценка: {}", grades.first());
    println!("Вторая оценка: {}", grades.second());

    let temperatures = Pair::new(18.0, 22.0);
    println!("Средняя температура: {}", temperatures.average());

    let article = Article {
        title: String::from("Ownership without tears"),
        author: String::from("Rust Team"),
    };

    let note = Note {
        text: String::from("Повторить trait bounds"),
    };

    print_summary(&article);
    print_summary(&note);

    let options = FeedOptions {
        limit: 5,
        ..Default::default()
    };

    println!("Компактный режим: {}", options.compact);
    println!("Лимит: {}", options.limit);

}