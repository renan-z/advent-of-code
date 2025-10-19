use regex::Regex;

pub fn parser(input: &str) -> Vec<&str> {
    let re = Regex::new(r"(?:mul|do_not_mul)\(\d{1,3},\d{0,3}\)").unwrap();
    let data = re.find_iter(input).map(|m| m.as_str()).collect();
    data
}

pub fn calulate(input: &str) -> u32 {
    let re = Regex::new(r"\d{1,3}").unwrap();
    let numbers = re
        .find_iter(input)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .reduce(|acc, x| acc * x)
        .unwrap_or(0);
    numbers
}

pub fn process(input: &str) -> String {
    let result: u32 = parser(input).iter().map(|s| calulate(s)).sum();
    result.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input))
    }
}
