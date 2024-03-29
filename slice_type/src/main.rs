fn main() {
    // let mut s = String::from("hello world");

    // let word = first_word(&s);  // word will get the value 5

    // for (i, c) in s.chars().enumerate() {
    //     if i <= word {
    //         println!("{c}");
    //     }
    // }

    // s.clear();  // this empties the String, making it equal to ""

    // for (i, c) in s.chars().enumerate() {
    //     if i <= word {
    //         println!("{c}");
    //     }
    // }
    // // word still has the value 5 here, but there's no more string that
    // // we could meaningfully use the value 5 with. word is now totally invalid!

    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];

    // println!("{hello}");
    // println!("{world}");

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("{}", word);
    let word = first_word(&my_string[..]);
    println!("{}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("{}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("{}", word);
    let word = first_word(&my_string_literal[..]);
    println!("{}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    println!("{}", word);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn _second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..]
        }
    }

    &s[..]
}