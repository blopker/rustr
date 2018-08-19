use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    println!("{:?}", read_username_from_file("hello"));
    println!("{:?}", read_username_from_file_easy("hello"));
    println!("{:?}", read_username_from_file_easist("hello"));
    file_fail();
}

fn vec_fail() {
    let v = vec![1, 3, 4];
    v[100];
}

fn file_fail() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => panic!("{:?}", e),
    };

    let f = File::open("bo.txt").expect("Cannot open, bo");
    let f = File::open("bo.txt").unwrap_or_else(|err| {
        panic!("Cannot open, {:?}", err);
    });
}

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let r = File::open(path);
    let mut f = match r {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => return Ok(s),
        Err(e) => return Err(e),
    }
}

fn read_username_from_file_easy(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_easist(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
