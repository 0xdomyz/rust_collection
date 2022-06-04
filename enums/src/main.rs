enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // enum

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    // enum variant associated values
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }
    
    // enum IpAddr2 {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    // let home = IpAddr2::V4(127, 0, 0, 1);
    // let loopback = IpAddr2::V6(String::from("::1"));

    // func
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // impl Message {
    //     fn call(&self) {
    //         // method body would be defined here
    //         // println!("{}","dasf");
    //     }
    // }

    // let m = Message::Write(String::from("hello"));
    // m.call();

    // option
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}

