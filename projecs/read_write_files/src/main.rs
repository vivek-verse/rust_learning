use std::fs::File;
use std::io::prelude::*;

fn main() {
    //create a file and write into it

    let mut file = File::create("info.txt").expect("Could not create file!");

    file.write_all(b"Hello World!")
        .expect("Cannot write to the file");

    //read the file

    let mut file = File::open("info.txt").expect("Can't open file!");
    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Oops! Can not read the file...");

    println!("Content of files are {:?}", contents);
}
