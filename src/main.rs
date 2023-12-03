use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Guess the number! Please input your guess");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        //shadowing lets us reuse the guess variable name rather than forcing us to 
        // create two variables, such as guess_str and guess.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //The underscore, _, is a catchall value
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
