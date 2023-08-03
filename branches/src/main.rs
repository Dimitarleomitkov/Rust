fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Rust throws an error if the condition does not evaluate to boolean
    // let number2 = 3;

    // if number2 {
    //     println!("number was three");
    // }

    let number3 = 3;

    if number3 != 0 {
        println!("number was something other than zero")
    }

    let number4 = 6;

    if number4 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number4 % 3 == 0 {
        println!("number is divisible by 3");
    } else if number4 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    let number5 = if condition { 5 } else { 6 };

    println!("The value of number is: {number5}");

    // To use this, the types of the values must be the same
    // let condition = true;

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");
}
