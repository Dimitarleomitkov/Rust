use std::env;
use std::process;

fn main() {
    // Access command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there is an argument for the degrees
    if args.len() < 2 {
        println!("Too few arguments. Give me degrees in F");
        process::exit(1);
    }
    let result: Result<f32, _> = args[1].parse();

    match result {
        Ok(number) => {
            println!("{}°F is {}°C", number, f_to_c(number));
        }
        Err(error) => {
            println!("Error parsing input: {}", error);
        }
    }
}

fn f_to_c(t: f32) -> f32 {
    5.0 / 9.0 * (t - 32.0)
}