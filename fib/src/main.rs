use std::io;

const MAX_INPUT : usize = 20;

fn main() {
    println!("Hello, world!");
    
    let mut input_number = String::new();

    io::stdin()
    .read_line(&mut input_number)
    .expect("Incorrect input");

    let input_number : usize = input_number.trim()
    .parse()
    .expect("Input value is not a number");

    if input_number >= MAX_INPUT{
        println!("Too big!!!");
        return;
    }

    let mut values = [0 ; MAX_INPUT];
    let result = fib(input_number, &mut values);

    println!("Result: {}", result);
}

fn fib(n : usize, mut values : &mut [u32; MAX_INPUT]) -> u32 {
    let result = if n == 0 {
        1
    } else if n == 1 {
        1
    } else {
        if values[n] != 0 {
            values[n]
        }else {
            let ans = fib(n - 1, &mut values) + fib(n - 2, &mut values);
            values[n] = ans;
            ans
        }
    };
    result
}