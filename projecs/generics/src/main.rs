use std::cmp::PartialOrd;

#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    println!("{}", find_largest(vec![10, 20, 4, 66, 33]));
    println!("{}", find_largest(vec!['a', 'f', 'b', 'e', 'c']));
    let _point1 = Point { x: 5, y: 10 };
    let _point2 = Point { x: "a", y: "z" };

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    println!("p1.x() {:?}", p1.x());
    println!("p1.y() {:?}", p2.y());
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
