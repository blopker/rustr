fn main() {
    let number_list = vec![2, 55, 2, 467, 236, 7];
    let char_list = vec!['d', 'f', 'z', 'a', 'b', 't'];
    point_test();
    println!("{}", find_largest(&number_list));
    println!("{}", find_largest(&char_list));
    println!("{}", find_largest_ref(&number_list));
    println!("{}", find_largest_ref(&char_list));
}

fn find_largest<T: PartialOrd + Copy>(lis: &[T]) -> T {
    let mut largest = lis[0];
    for &i in lis.iter() {
        if i > largest {
            largest = i
        }
    }
    largest
}

fn find_largest_ref<T: PartialOrd>(lis: &[T]) -> &T {
    let mut largest = &lis[0];
    for i in lis.iter() {
        if i > largest {
            largest = i
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

impl Point<usize> {
    pub fn distance_from_origin(&self) -> f32 {
        ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
    }
}

fn point_test() {
    let a = Point { x: 1.2, y: 3.0 };
    let b = Point { x: 1, y: 3 };
    println!("{:?}", a);
    println!("{:?}", b.x);
    println!("{:?}", b.distance_from_origin());
}
