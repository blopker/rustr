use std::collections::HashMap;

const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);
    println!("max is {}", MAX_POINTS);

    let y = 1;
    println!("y is {}", y);
    let y = y + 1;
    println!("y is {}", y);
    let y = y * 10;
    println!("y is {}", y);

    tuples();
    arrays(100);
    println!("expressions {}", expressions(4, 5));
    loops();

    let mut cache: HashMap<usize, usize> = HashMap::new();
    println!("{}", fib(93, &mut cache));
}

fn tuples() {
    let b: (u8, u32, f64) = (1, 3000, 1.6);
    println!("b.1 {}", b.1);
    let (_, _, s) = b;
    println!("b.2 {}", s);
}

fn arrays(x: usize) {
    let a = [x, 2, 3];
    println!("a.2 {}", a[2]);
}

fn expressions(y: u32, z: u32) -> u32 {
    let d = {
        let f = y + z;
        f - 1
    };
    let x = if d < 5 {
        d
    } else if d < 9 {
        // this returns
        d * 4
    } else {
        0
    };
    x * 10
}

fn loops() {
    let mut a = 0;
    loop {
        println!("looop");
        a += 1;
        if a == 3 {
            break;
        }
    }
    let mut x = 0;
    while x < 2 {
        println!("while {}", x);
        x += 1
    }

    let a = [1, 2, 3, 4];
    for i in &a {
        println!("array {}", i);
    }

    for i in (1..4).rev() {
        println!("range {}", i);
    }
}

fn fib(n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    let answer = match cache.get(&n) {
        Some(&x) => x,
        None => {
            if n == 0 {
                0
            } else if n == 1 {
                1
            } else {
                fib(n - 1, cache) + fib(n - 2, cache)
            }
        }
    };
    cache.insert(n, answer);
    answer
}
