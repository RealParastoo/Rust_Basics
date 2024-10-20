
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("anotheremail@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("user1email@example.com");
    
    println!("{0}", user1.active);
    println!("{0}", user1.username);
    println!("{0}", user1.email);
    println!("{0}\n", user1.sign_in_count);

    let user2 = build_user("user2email@example.com".to_string(), "user2".to_string());
    
    println!("{0}", user2.active);
    println!("{0}", user2.username);
    println!("{0}", user2.email);
    println!("{0}", user2.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 2,
    }
}
