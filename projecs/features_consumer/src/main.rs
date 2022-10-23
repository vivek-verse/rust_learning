use looking_into_features::color;
fn main() {
    looking_into_features::draw_line(32, 32);

    let color = color::RGB {
        r: 247,
        g: 76,
        b: 0,
    };

    color::draw_line(32, 32, &color);

    let square = looking_into_features::shapes::Rectangle {
        color,
        width: 32,
        height: 32,
    };

    println!("{square:?}");
}
