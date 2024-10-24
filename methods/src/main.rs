struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
      self.width * self.height  
    }

    fn width(&self)->bool{
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect1 = Rectangle{
        width:20,
        height:10,
    };

    let rect2 = Rectangle{
        width:12,
        height:10,
    };

    let rect3 = Rectangle{
        width:45,
        height:38,
    };

    let sq = Rectangle::square(9);

    println!("The area of rect1 is: {}",rect1.area());
    println!("Is rect1 width nonzero? {}",rect1.width());
    println!("Can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}",rect2.can_hold(&rect3));
    println!("Can rect3 hold rect2? {}",rect3.can_hold(&rect2));

}
