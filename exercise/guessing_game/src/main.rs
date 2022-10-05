use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin() // stdin gives the handle to current process input stream
            .read_line(&mut guess) // read_line gives Result output so we can use expect on it
            .expect("Error in reading the input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
