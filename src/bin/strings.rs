fn main() {
    let mut s = String::new();
    s.push_str("im string");
    let b = "im string".to_string();
    let c = String::from("im string");

    assert_eq!(s, b);
    assert_eq!(s, c);

    let hello = String::from("Здравствуйте");

    let d = "im ".to_string();
    let f = "string".to_string();
    let g = d + &f;
    assert_eq!(s, g);
    let r = format!("{}-{}", g, hello);
    println!("{}", r);

    println!("{}", String::from("Здравствуйте").len()); // 24

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    println!("{}", "नमस्ते".chars()[2]) //Strings Are Not So Simple
}
