use std::io;

#[derive(Debug)]
struct Rectangle {
    height: usize,
    width: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.height * self.width
    }
    fn double(&mut self) {
        self.height = self.height * 2;
        self.width = self.width * 2;
    }
    fn square(size: usize) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// multiple impl blocks is ok
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height >= other.height && self.width >= other.width
    }
}

fn main() {
    let num_height = get_input("enter height");
    let num_width = get_input("enter width");

    let mut rect = Rectangle {
        height: num_height,
        width: num_width,
    };

    println!("rect is: {:?}", rect);
    println!("rect is {:#?}", rect);
    println!("Area is {}", rect.area());

    rect.double();
    println!("rect is {:#?}", rect);
    println!("Area is {}", rect.area());

    let rect2 = Rectangle {
        width: 5,
        height: 4,
    };

    println!("can hold? {}", rect.can_hold(&rect2));

    let square = Rectangle::square(10);
    println!("im a square {:#?}", square);
}

fn get_input(message: &str) -> usize {
    // TODO figure out how to pass a type
    let num: usize;
    loop {
        let mut buffer = String::new();
        println!("{}", message);
        io::stdin().read_line(&mut buffer).expect("be buffer");
        num = match buffer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    num
}
