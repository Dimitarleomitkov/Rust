fn main() {
    let x = 2.0;    // f64

    let y: f32 = 3.0;   // f32

    println!("{x}, {y}");

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!("5 + 10 = {sum}");
    println!("95.5 - 4.3 = {difference}");
    println!("4 * 30 = {product}");
    println!("56.7 / 32.2 = {quotient}");
    println!("2 / 3 = {floored}");
    println!("43 % 5 = {remainder}");

    let t = true;
    let f: bool = false;    // with explicit type annotation

    println!("{t}, {f}");

    let c = 'z';
    let z: char = 'Z';  // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c}   {z}   {heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 2);


    let (x, y, z) = tup;
    println!("({x}, {y}, {z})");
    let (x, y, z) = tup2;
    println!("({x}, {y}, {z})");
    println!("({}, {}, {})", tup.0, tup.1, tup.2);

    let a = [1, 2, 3, 4, 5];

    println!("{}", a[0]);

    let months = ["January", "February", "March", "April", "May", "June",
                    "July", "August", "September", "October", "November", "December"];

    println!("{}", months[0]);

    let b: [i32; 5] = [5, 6, 7, 8, 9];

    println!("{}", b[3]);

    let c = [3; 5];

    println!("{}, {}, {}, {}, {}", c[0], c[1], c[2], c[3], c[4]);
}
