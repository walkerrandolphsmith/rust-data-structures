pub fn add_two(a: i32) -> i32 {
	a + 2
}

#[cfg(test)]
mod tests {
	use super::add_two;

	fn should_add_two() {
		assert_eq!(4, add_two(2));
	}

    #[test]
    #[should_panic]
    fn should_panic() {
        assert!(false);
    }

    #[test]
    fn should_not_panic() {
        assert!(true);
    }
    
    #[test]
    fn equality() {
        assert_eq!("Hello", "Hello");
    }
    
    #[test]
    #[should_panic(expected = "assertion failed")]
    fn equality_should_panic() {
        assert_eq!("Hello", "World");
    }
    
    #[test]
    #[ignore]
    fn ignored_test() {
        assert!(false);
    }
}
