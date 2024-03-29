struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };

    println!("{}", user1.email);

    user1.email = String::from("anotheremail@example.com");

    println!("{}", user1.email);

    let user2 = build_user(String::from("asd@example.com"), String::from("xXxSasukexXx"));

    println!("\nactive: {}\n\
        username: {}\n\
        email: {}\n\
        sign_in_count: {}",
        user2.active,
        user2.email,
        user2.username,
        user2.sign_in_count);

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count
    };

    let user4 = User{
        email: String::from("justemail@exampl.com"),
        ..user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{}", black.1);
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1
    // }

    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}