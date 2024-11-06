
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}
enum IpAddrKind1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

//let home = IpAddrKind1::V4(192, 168, 3, 10);
//let loopback = IpAddrKind1::V6(String::from("::1"))

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


fn main() {
    let ret;

    ret = value_in_cents(Coin::Nickel);
    println!("{ret}");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
