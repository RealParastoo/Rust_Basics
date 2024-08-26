fn main() {
    let mut x = 6;
    println!("x is: {x}");
    x = 7;
    println!("x is: {x}");
    let mut _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);
    _immutable_binding += 1;
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);
}
