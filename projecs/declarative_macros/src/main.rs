use declarative_macros::*;
use std::collections::HashMap;
fn main() {
    let v1: Vec<u32> = vivek![1, 2, 3];
    println!("v1 is {:?}", v1);
    let mut scores = map!(String, i32);
    scores.insert("Red team".to_owned(), 3);
    scores.insert("Blue team".to_owned(), 5);
    scores.insert("Green team".to_owned(), 2);

    println!("{scores:?}");

    let scores2 = map!(
        "Red team".to_owned() => 3,
        "Blue team".to_owned() => 5,
        "Green team".to_owned() => 2
    );

    println!("{scores2:?}");
}
