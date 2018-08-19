use std::fmt::Display;

fn longest<'a>(one: &'a str, two: &'a str) -> &'a str {
    if one.len() > two.len() {
        one
    } else {
        two
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Exerpt<'a> {
    part: &'a str,
}

impl<'a> Exerpt<'a> {
    fn level(&self) -> usize {
        3
    }
    fn first_part(&self) -> &str {
        &self.part[0..1]
    }
    fn n_part(&self, n: usize) -> &str {
        &self.part[..n]
    }
}

fn main() {
    let one = String::from("imastring");
    let two = "alsoastring";
    let result = longest(one.as_str(), two);
    println!("{}", result);

    let exerpt = String::from("Call me Ishmael. Some years ago...");
    let first = exerpt.split('.').next().expect("needs moar");
    let i = Exerpt { part: first };
    println!("{:?}", i);
    println!("{}", i.level());
    println!("{}", i.first_part());
    println!("{}", i.n_part(3));
    println!(
        "{}",
        longest_with_an_announcement(one.as_str(), two, "announce!")
    );
}
