pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_failed(left: u64, right: u64) -> u64 {
    left + right + 1
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

// cargo test
#[cfg(test)]
mod testmod {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4, "2 + 2 should be 4");
    }

    #[test]
    fn it_not_works() {
        let result = add_failed(2, 2);
        assert_eq!(result, 4, "2 + 2 should be 4");
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
