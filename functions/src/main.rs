fn main() {
    // Statement
    let y = 6;

    println!("y = {}", y);

    // Statements do not return values
    // let x = (let y = 6);

    // 6 is by itself an expression
    // Calling a function is an expression
    // Calling a macro is an expression
    // A new scope block {} is an expression

    let z = {
        let x = 3;
        // Only statements end with a ;
        x + 1
    };

    println!("The value of z is {z}");

    another_function(5);
    print_labeled_measurement(5, 'h');

    let t_var = five();

    println!("The value of t_var is: {t_var}");

    println!("Using the function plus_one with 5: {}", plus_one(5));
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}