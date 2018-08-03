use std::io;

fn main() {
    let num_height: usize;
    loop {
        let mut height = String::new();
        println!("enter height");
        io::stdin().read_line(&mut height).expect("Be height");
        num_height = match height.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }

    let num_width: usize;
    loop {
        let mut width = String::new();
        println!("enter width");
        io::stdin().read_line(&mut width).expect("be width");
        num_width = match width.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        break;
    }

    let area = num_width * num_height;

    println!("Area is: {}", area);
}
