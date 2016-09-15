fn get_hello_world_message() -> &'static str {
    "Hello, World!"
}

fn main() {
    println!("{}", get_hello_world_message());
}

#[test]
fn prints_hello_world() {
    assert_eq!("Hello, World!", get_hello_world_message());
}
