// enum IpAddrKind {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// struct Ipv4Addr {
//     // --snip--
// }

// struct Ipv6Addr {
//     // --snip--
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// enum Message {
//     Quit,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

fn main() {
    // let four = IpAddrKind::V4(127,0,0,1);
    // let six = IpAddrKind::V6(String::from("::1"));

    // route(four);
    // route(six);

    // let m = Message::Write(String::from("hello"));
    // m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}

// fn route(ip_kind: IpAddrKind) {
    
// }