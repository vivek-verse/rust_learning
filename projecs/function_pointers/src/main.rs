fn main() {
    let greater_than = |x: &i32| *x > 10;
    let result = are_both_true(greater_than, less_than, &9);
    println!("{result}");
}

fn less_than(x: &i32) -> bool {
    *x < 20
}

// A closure coerces to function pointer if it does not owns a value

fn are_both_true<T, V>(f1: T, f2: fn(&V) -> bool, item: &V) -> bool
where
    T: Fn(&V) -> bool,
{
    f1(item) && f2(item)
}
