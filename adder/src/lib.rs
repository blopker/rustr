use std::ops::Add;

pub struct Rectangle {
    length: usize,
    width: usize,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length >= other.length && self.width >= other.width
    }
}

pub fn add_two<T>(x: T, y: T) -> T
where
    T: Add<Output = T>,
{
    x + y
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;
    #[test]
    fn explore() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_can_hold() {
        let r1 = Rectangle {
            length: 200,
            width: 300,
        };
        let r2 = Rectangle {
            length: 100,
            width: 300,
        };
        assert!(r1.can_hold(&r2));

        let r1 = Rectangle {
            length: 200,
            width: 300,
        };
        let r2 = Rectangle {
            length: 300,
            width: 300,
        };
        assert!(!r1.can_hold(&r2))
    }

    #[test]
    fn add_ints() {
        assert_eq!(add_two(1, 2), 3);
    }
    #[test]
    fn add_floats() {
        assert_eq!(add_two(1.1, 2.3), 3.4);
    }
}
