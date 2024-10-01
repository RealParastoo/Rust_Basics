fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    let x= 5;
    makes_copy(x);

    let _s1 = gives_ownership();                                          
    let s2 = String::from("hello");   
    let _s3 = takes_and_gives_back(s2); 
    
    let s4 = String::from("test");

    let (s5, len) = calculate_length_ownership(s4);
    println!("The length of '{s5}' is {len}.");

    let s4 = String::from("program");
    let length = calculate_length(&s4);
    println!("The length of '{s4}' is {length}.");
}

fn takes_ownership(some_string: String)
{
    println!("{some_string}");
}

fn makes_copy(some_int: i32)
{
    println!("{some_int}");
}

fn gives_ownership() -> String { 

    let some_string = String::from("yours");
    some_string                              
}

fn takes_and_gives_back(a_string: String) -> String { 

    a_string  
}

fn calculate_length_ownership(s: String) -> (String, usize) {
    let length = s.len(); 

    (s, length)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}