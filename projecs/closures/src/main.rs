/*
    3 traits
    Fn - immutable
    FnMut - mutable reference
    FnOnce - By move (taken total ownership)
*/

fn add_five<F>(mut func: F)
where
    F: FnMut(i32),
{
    func(5)
}

fn main() {
    let mut str = "The quick brown fox".to_string();
    let clzr1 = |x| println!("{} {}", str, x);
    clzr1("jumps over the lazy dog"); //Fn Trait

    let mut clzr2 = |x| str.push_str(x);
    clzr2(" jumps over the lazt dog");
    println!("{}", str); //FnMut trait

    let clzr3 = || drop(str);

    let mut num = 6;

    add_five(|x| {
        num += x;
        println!("{}", num)
    });

    let car_collection = vec![
        ("Thunderbird", 1960),
        ("Cobra", 1966),
        ("GT", 1967),
        ("Mustang Grande", 1969),
    ];

    let ford_models = ClassicCars {
        make: "Ford",
        models: car_collection,
    };

    ford_models.smart_get(|x| {
        let res: Vec<_> = x.into_iter().filter(|x| x.1 > 1960).collect();
        println!("results {:?}", res);
    });
}

struct ClassicCars {
    make: &'static str,
    models: Vec<(&'static str, i32)>,
}

impl ClassicCars {
    fn smart_get<F>(&self, f: F)
    where
        F: Fn(&Vec<(&'static str, i32)>),
    {
        f(&self.models)
    }
}
