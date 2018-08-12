extern crate communicator;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

pub mod one {
    pub mod two {
        pub mod three {
            pub fn afunction() {}
        }
    }
}

#[cfg(test)]
mod test {
    use super::one::two::three;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
        super::one::two::three::afunction();
        ::one::two::three::afunction();
        three::afunction();
    }
}

use one::two::three;
use TrafficLight::{Green, Red};

fn main() {
    one::two::three::afunction();
    three::afunction();
    let a = Red;
    let b = Green;
    communicator::client::connect();
}
