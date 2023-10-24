fn main() {
    // {                      // s is not valid here, it’s not yet declared
    //     let s = "hello";   // s is valid from this point forward

    //     // do stuff with s
    // }                      // this scope is now over, and s is no longer valid

    // let s = String::from("hello");
    let mut s = String::from("Hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    // println!("{}", s); // This will print `Hello, world!`
    println!("{s}"); // This will print `Hello, world!`

    // {
    //     let s = String::from("hello"); // s is valid from this point forward

    //     // do stuff with s
    // }                                  // this scope is now over, and s is no
    //                                    // longer valid

    // let x = 5;
    // let y = x;

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2. s1 is no longer valid

    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // If we actually want to make a copy:
    let s3 = String::from("Hola");
    let s4 = s3.clone();

    println!("s3 = {s3}, s4 = {s4}");

    let s5 = String::from("Yo!");   // s5 comes into scope

    takes_ownership(s5);            // s5's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let s6 = gives_ownership();     // gives_ownership moves its return
                                    // value into s6

    let s7 = takes_and_gives_back(s4);   // s4 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s7

    let (s8, len) = calculate_length(s7);

    println!("{s6}, {s8}, {len}");
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.
  // Here, s7 goes out of scope and is dropped. s4 was moved, so nothing
  // happens. s6 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}")
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {                // gives_ownership will  move its
                                                // return value into the function
                                                // that calls it

    let some_string = String::from("yours");    // some_string comes into scope

    some_string                                 // some_string is returned and
                                                // moves out to the calling
                                                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into
                                                        // scope

    a_string    // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();   // len() returns the length of a String

    (s, length)
}