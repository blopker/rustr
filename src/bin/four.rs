fn main() {
    let mut s = String::from("bo is awesome 🦅");
    s.push_str("🦉");
    println!("{}", s);
    let s1 = String::from("bobobo");
    let s2 = s1.clone();
    println!("{} {}", s1, s2);
    own_me(s2);
    let mut s1 = give_back(s1);
    println!("{}", calculate_length(&s1));
    append_bo(&mut s1);
    println!("{}", s1);
}

fn own_me(s: String) {
    println!("i own s {}", s);
}

fn give_back(s: String) -> String {
    s
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn append_bo(s: &mut String) {
    s.push_str("BO")
}

fn first_word(s: &str) -> &str {
    
}