use std::collections::HashMap;

fn main() {
    println!("bobobo");
    let mut b = HashMap::new();
    b.insert("bo", "isawesome");
    b.insert("bo", "is super");
    b.entry("nothere").or_insert("notishere");
    println!("{:?}", b);

    let teams = vec!["red".to_string(), "blue".to_string()];
    let initial_scores = vec![10, 20];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (k, v) in &scores {
        println!("{}-{}", k, v);
    }
    println!("{}", scores.get(&teams[0]).expect("there"));

    moved();
    count();
}

fn moved() {
    let mut b = HashMap::new();
    let bo = "bo".to_string();
    let bobo = "bobo".to_string();
    b.insert(bo, bobo);
    // println!("{}", bo);
}

fn count() {
    let text = "hello world wonderful world";
    let mut counter = HashMap::new();
    for word in text.split_whitespace() {
        let e = counter.entry(word).or_insert(0);
        *e += 1;
    }
    println!("{:?}", counter);
}
