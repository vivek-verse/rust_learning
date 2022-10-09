use declarative_macros::vivek;
fn main() {
    let v1: Vec<u32> = vivek![1, 2, 3];

    println!("v1 is {:?}", v1);
}
