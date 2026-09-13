fn greet(name: &str) -> String {
    format!("hello, {name}!")
}

fn main() {
    println!("{}", greet("merlin"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_greet() {
        assert_eq!(greet("merlin"), "hello, merlin!");
    }
}
