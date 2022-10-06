fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let check_result;

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    {
        let string3 = String::from("tuv");
        //here string3 will no longer exist after this scope but since we return x with lifetime of x so it will work!
        check_result = check(string1.as_str(), string3.as_str());
    }

    println!("Check result is : {}", check_result);
}

fn check<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
