use std::io;

fn main() {
    println!("Hello, world!");

    let mut guess = String::new();

    let num_bytes = io::stdin()
                    .read_line(&mut guess)
                    .expect("Wow! error! It's a wonderful day!!!");

    println!("You guessed : {}Length : {}", guess, num_bytes);
}
