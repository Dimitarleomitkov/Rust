// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.")
// }

// fn calculate_length(s: &String) -> usize {  // s is a reference to a String
//     s.len()
// }   // Here, s goes out of scope. But because it does not have ownership of what
//     // it refers to, it is not dropped.

// fn main() {
//     let mut s = String::from("DZE");

//     change(&mut s);
//     println!("{s}");
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(" WORLDO!");
// }

fn main() {
    let mut s = String::from("TZE");

    let r1 = &s;    // no problem
    let r2 = &s;    // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s;    // no problem
    println!("{r3}");
    // If r3 is declared after r2, before the println!(), the code will not compile

    // Dangling reference
    // let reference_to_nothing = dangle(); 
}

// fn dangle() -> &String {    // dangle returns a reference to a String
//     let s = String::from("hello");  // s is a new String

//     &s  // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!