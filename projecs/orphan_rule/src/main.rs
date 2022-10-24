use orphan_rule::Point;

// impl PartialEq for Point {
//     fn eq(&self, other: &Self) -> bool {
//         self.x == other.x && self.y == other.y
//     }
// }

//Commented code will not work because Point is not defined in the current crate
// workaround is to create a wrapper

struct PointWrapper(Point);

impl PartialEq for PointWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.x == other.0.x && self.0.y == other.0.y
    }
}

fn main() {
    let p1 = PointWrapper(Point { x: 1, y: 2 });
    let p2 = PointWrapper(Point { x: 1, y: 2 });
    println!("{}", p1 == p2);
}
