
pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 amd 100, got {}.", value);
        }
        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn it_works_r() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal to four"))
        }
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was: {}", result
        )
    }
    #[test]
    #[ignore]
    fn greeting_contains_name2() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol0"),
            "Greeting did not contain name, value was: {}", result
        )
    }
    #[test]
    #[should_panic(expected = "Guess value must be between 1 amd 100")] // panic 子串
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    #[ignore] // 排除
    fn another() {
        panic!("Make this test fail")
    }
}
