fn announce(project: String) -> String {
    println!("Проект: {project}");
    project
}

fn add_suffix(text: String) -> (String, String) {
    let tagged = format!("{text}-v1");
    (text, tagged)
}

fn main() {
    let x = 42;
    let y = x;

    let project = String::from("calculator");
    let project = announce(project);
    println!("Длина project: {}", project.len());

    let release = String::from("rust");
    let (release, tagged) = add_suffix(release);
    println!("Исходная метка: {release}");
    println!("Новая метка: {tagged}");

    let label = String::from("move");
    let label_copy = label.clone();
    println!("label = {label}, label_copy = {label_copy}");

    println!("x = {x}, y = {y}");
}