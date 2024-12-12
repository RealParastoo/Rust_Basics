
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

    let x = 7u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
