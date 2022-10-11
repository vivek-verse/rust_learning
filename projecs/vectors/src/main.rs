fn main() {
    let v = vec![1, 2, 3, 4, 5];

    match v.get(19) {
        Some(nineteen) => println!("Nineteen came is {}", nineteen),
        None => println!("There is no nineteen"),
    }

    for num in &v {
        println!("num came is {}", num);
    }

    println!("v before is {:?}", v);

    let result = v.iter().map(|x| x + 2).collect::<Vec<i32>>();

    println!("result is {:?}", result);
}
