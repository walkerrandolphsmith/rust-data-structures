

pub fn get_hello_world_message() -> &'static str {
    "Hello, World!"
}

fn main() {
    println!("{}", get_hello_world_message());
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn prints_hello_world() {
        assert_eq!("Hello, World!", get_hello_world_message());
    }
}
