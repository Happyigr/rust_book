#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            heigth: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.heigth
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.heigth > other.heigth
    }
}

fn main() {
    let scale = 30;
    let sqr1 = Rectangle::square(3);
    let rect1 = Rectangle {
        width: dbg!(scale * 30),
        heigth: 50,
    };
    let rect2 = Rectangle {
        width: dbg!(scale * 20),
        heigth: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Area is {}", rect1.area());
    println!("rect1 is {:?}", rect1);
    println!("rect2 is {:#?}", rect2);
    println!("sqr1 is {:#?}", sqr1);
    //dbg! returns the owonership of value back to the owner
    dbg!(rect1);
}
