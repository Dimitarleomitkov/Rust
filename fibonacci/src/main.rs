use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if there is an argument for the number
    if args.len() < 2 {
        println!("Too few arguments. Give me n for the Fibonacci number");
        process::exit(1);
    } 

    // Parse the argument
    let result: Result<u64, _> = args[1].parse();
    match result {
        Ok(number) => {
            if number > 93 {
                println!("This will overflow the number. Choose another n");
                process::exit(0);
            }

            for i in 0..number {
                print!("{}, ", fibonacci(i));
            }
        }
        Err(error) => {
            println!("Error parsing input: {}", error);
        }
    }
}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return 1 as u64;
    }

    let mut prev = 1;
    let mut current = 1;

    for _ in 2..=n {
        let next = prev + current;
        prev = current;
        current = next;
    }

    current
}