use std::collections::HashMap;
fn main() {
    let blue = String::from("Blue");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);

    let team_blue = String::from("Blue");

    match scores.get(&team_blue) {
        Some(score) => println!("score is {}", score),
        None => println!("No such index"),
    }

    //to insert an entry if not exists
    scores.entry(String::from("Yellow")).or_insert(30);
    println!("scores hashmap is : {:?}", scores);
}
