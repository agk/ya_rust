struct Lesson {
    title: String,
    notes: String,
    views: u32,
}

impl Lesson {
    fn new(title: String, notes: String) -> Self {
        Self {
            title,
            notes,
            views: 0,
        }
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn notes_len(&self) -> usize {
        self.notes.len()
    }

    fn add_note(&mut self, extra: &str) {
        self.notes.push_str(extra);
    }

    fn view(&mut self) {
        self.views += 1; 
    }

    fn publish(self) -> String {
        format!("Урок '{}' опубликован", self.title)
    }

}

fn print_title(title: &str) {
    println!("{}", title)
}

fn main() {
    let mut lesson = Lesson::new("урок 1".to_string(), "".to_string());
    print_title(&lesson.title);
    println!("{}", lesson.notes_len());
    lesson.add_note(" еще один фрагмент");
    lesson.view();
    lesson.publish();

}
