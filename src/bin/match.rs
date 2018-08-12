#[derive(Debug)]
enum UsState {
    Alabama,
    California,
    Maine,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<usize>) -> Option<usize> {
    match x {
        None => None,
        Some(n) => Some(n + 1),
    }
}

fn speak(x: u8) {
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("TODO"),
    }
}

fn lazy_match(x: Option<u8>) {
    if let Some(0u8) = x {
        println!("its 0");
    } else {
        println!("not 0");
    }
}

fn main() {
    println!("value {}", value_in_cents(Coin::Penny));
    println!("value {}", value_in_cents(Coin::Nickle));
    println!("value {}", value_in_cents(Coin::Dime));
    println!(
        "value {}",
        value_in_cents(Coin::Quarter(UsState::California))
    );
    let a = 2;
    println!("plus one {} {}", a, plus_one(Some(a)).expect("num"));
    speak(1);
    speak(10);
    lazy_match(Some(0u8));
}
