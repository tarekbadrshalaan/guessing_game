use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guess the number game!");
    loop {
        println!("Please input your guess!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let secret_number = rand::thread_rng().gen_range(0, 10);
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!(
                    "Please type a number! your input:{} error : {}",
                    guess.trim(),
                    err
                );
                continue;
            }
        };

        //
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win :)");
                break;
            }
        }
        println!("secret number :{}", secret_number);
    }
}
