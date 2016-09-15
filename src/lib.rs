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
