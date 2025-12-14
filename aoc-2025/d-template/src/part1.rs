pub fn process(input: &str) -> String {
    println!("Input 1: {}", input);
    "Hello".to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Hello";
        assert_eq!("Hello", process(input))
    }
}