#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn length(&self) -> u32 {
        2*(self.width + self.height)
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}



fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "The length of the rectangle is {} long. ",
        rect1.length()
    );
    let rect2 = Rectangle::square(30);
    
    println!(
        "The area of the rectangle2 is {} square pixels.",
        rect2.area()
    );
}