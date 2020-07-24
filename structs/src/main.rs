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
    fn can_hold(&self, subject: &Rectangle) -> bool {
        self.width >= subject.width && self.height >= subject.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1:\n{:#?}", rect1);
    println!("the area of rect1: {}", rect1.area());

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("can rect2 fit into rect1? {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle {
        width: 31,
        height: 51,
    };

    println!("can rect3 fit into rect1? {}", rect1.can_hold(&rect3));
}
