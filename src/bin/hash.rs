use std::collections::HashMap;

fn main() {
    println!("bobobo");
    let mut b = HashMap::new();
    b.insert("bo", "isawesome");

    let teams = vec!["red".to_string(), "blue".to_string()];
    let initial_scores = vec![10, 20];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}
