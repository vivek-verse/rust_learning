use std::cmp::PartialOrd;

#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    println!("{}", find_largest(vec![10, 20, 4, 66, 33]));
    println!("{}", find_largest(vec!['a', 'f', 'b', 'e', 'c']));
    let _point1 = Point { x: 5, y: 10 };
    let _point2 = Point { x: "a", y: "z" };
}

fn find_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for l in list {
        if l > largest {
            largest = l;
        }
    }
    largest
}
