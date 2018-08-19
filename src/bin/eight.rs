use std::collections::HashMap;

#[derive(Debug)]
struct Stats {
    average: f32,
    median: f32,
    mode: usize,
}

fn main() {
    println!("{:?}", stats(&[1, 2, 2, 3, 4, 7, 9]));
    println!("{:?}", stats(&[1, 2, 3, 4, 5, 6, 8, 9]));
    println!("{:?}", stats(&[1, 1, 2, 4, 5, 32, 7, 8, 4]));
    println!(
        "{:?}",
        stats(&[1, 1, 23, 4, 4, 32, 666, 22342, 456456, 23342, 666, 666])
    );
}

// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position), and mode
// (the value that occurs most often; a hash map will be helpful here) of the list.
fn stats(array: &[usize]) -> Stats {
    let mut vec = Vec::from(array);
    vec.sort_unstable();

    // average
    let mut sum = 0;
    for a in &vec {
        sum += a;
    }
    let average = (sum / vec.len()) as f32;

    // median
    let median: f32;
    if vec.len() % 2 == 0 {
        let middle = vec.len() / 2;
        median = (vec[middle] + vec[middle - 1]) as f32 / 2.0;
    } else {
        median = vec[(vec.len() / 2)] as f32;
    }

    // mode
    let mut counter = HashMap::new();
    for a in &vec {
        let mut ra = counter.entry(a).or_insert(0);
        *ra += 1
    }
    let mut mode = 0;
    let mut max = 0;
    for (k, v) in counter {
        if v > max {
            max = v;
            mode = *k;
        }
    }

    Stats {
        median,
        average,
        mode,
    }
}
