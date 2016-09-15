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
