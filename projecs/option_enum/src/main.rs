fn main() {
    let index: usize = 3;
    let name = String::from("Vivek");
    println!(
        "Character at index {} is: {}",
        index,
        match name.chars().nth(index) {
            Some(c) => c.to_string(),
            None => "No character at index 3!".to_string(),
        }
    );

    println!(
        "Occupation is {}",
        match get_occupation("Someone") {
            Some(o) => o,
            None => "Berozgaar",
        }
    )
}

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Vivek" => Some("Software Developer"),
        "I don't know" => Some("I don't know"),
        _ => None,
    }
}
