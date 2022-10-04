extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    is_male: bool,
}

use serde_json::Value as JsonValue;
fn main() {
    let json_str = r#"
        {
            "name" : "Vivek",
            "age" : 28,
            "is_male" : true
        }
    "#;

    let res = serde_json::from_str(json_str);

    if res.is_ok() {
        let p: Person = res.unwrap();
        println!("The name is {}", p.name);
    } else {
        println!("Sorry! Could not parse JSON :(");
    }
}
