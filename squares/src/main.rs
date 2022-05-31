#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
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

    let rect3 = Rectangle {
        width: 20,
        height: 45,
    };
    
    let sq = Rectangle::square(15);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("rect1 is {:?}", rect1);
    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("sqaure is {:?}",sq);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}



