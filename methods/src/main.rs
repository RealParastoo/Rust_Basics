struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
      self.width * self.height  
    }
}

fn main() {
    let rect1 = Rectangle{
        width:20,
        height:10
    };

    println!("The area of rect1 is: {}",rect1.area());
}
