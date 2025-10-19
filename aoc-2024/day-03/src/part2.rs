use regex::Regex;

pub fn parser(input: &str) -> Vec<&str> {
    let re = Regex::new(r"(?:mul\(\d{1,3},\d{0,3}\)|don't\(\)|do\(\))").unwrap();
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
    let mut enable = true;
    let re_enable = Regex::new(r"(?:do\(\))").unwrap();
    let re_disable = Regex::new(r"(?:don't\(\))").unwrap();

    let result: u32 = parser(input)
        .iter()
        .map(|s| {
            re_enable.is_match(s).then(|| enable = true);
            re_disable.is_match(s).then(|| enable = false);
            if enable { calulate(s) } else { 0 }
        })
        .sum();

    result.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input))
    }
}
