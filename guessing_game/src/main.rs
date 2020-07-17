use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let rand_num = rand::thread_rng().gen_range(1, 101);
    println!("Guess the secret number");
    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Error at read_line() : Cannot read line");

        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number");
                continue;
            },
        };
                            

        println!("You guessed : {}", guess);
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small!!!"),
            Ordering::Greater => println!("Too big!!!"),
            Ordering::Equal => {
                println!("Congraturations!!!");
                break;
            },
        }
    }
}
