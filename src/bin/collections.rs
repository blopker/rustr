enum Cell {
    Int(u32),
    Str(String),
    Float(f32),
}

fn main() {
    let v: Vec<u32> = Vec::new();
    let mut b = vec![1, 2, 3];
    b.push(1);
    b.push(10);

    {
        let third: &i32 = &b[2];
        let a = match b.get(2) {
            Some(n) => n,
            None => &0,
        };
        assert_eq!(a, third);
    }

    for i in &mut b {
        *i = *i * 10;
        println!("{}", i);
    }

    let row = vec![
        Cell::Int(2),
        Cell::Float(1.2),
        Cell::Str(String::from("bobob")),
    ];
}
