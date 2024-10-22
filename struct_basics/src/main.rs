
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // An attribute to hide warnings for unused code.
    #![allow(dead_code)]
    
    /*block 1*/
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("anotheremail@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("user1email@example.com");
    
    // println!("{0}", user1.active);
    // println!("{0}", user1.username);
    // println!("{0}", user1.email);
    // println!("{0}\n", user1.sign_in_count);

    let user2 = build_user("user2email@example.com".to_string(), "user2".to_string());
    
    // println!("{0}", user2.active);
    // println!("{0}", user2.username);
    // println!("{0}", user2.email);
    // println!("{0}", user2.sign_in_count);
    /*block 1*/

    /*block 2*/
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    struct Unit;
    struct Pair(i32, f32);
    struct Point {
        x: f32,
        y: f32,
    }
    struct Rectangle {
        top_left: Point,
        bottom_right: Point,
    }

    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    let another_point: Point = Point { x: 5.2, y: 0.2 };

    println!("point coordinates: ({}, {})", point.x, point.y);
    let bottom_right = Point { x: 5.2, ..another_point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };


    let _unit = Unit;
    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
    /*block 2*/
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 2,
    }
}
