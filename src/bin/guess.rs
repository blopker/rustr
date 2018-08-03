extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Time to guess numbers. You know what to do.");
    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter a number");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("fail");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        match guess.cmp(&secret){
            Ordering::Less => println!("Too small peepee"),
            Ordering::Greater => println!("Too big poopoo"),
            Ordering::Equal => {
                    println!("Dope");
                    break;
                },
        }
    }
    println!("You win!");
}
