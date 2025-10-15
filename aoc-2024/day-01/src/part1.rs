use regex::Regex;

pub fn parser(input: &str) -> String {
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    let mut sum = 0;
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let a = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let b = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        // Add first number to first list
        // Add second number to second list
        first_list.push(a);
        second_list.push(b);
    }
    // Sort by increase
    first_list.sort();
    second_list.sort();
    // Get sum of difference of item from first and second list
    for i in 0..first_list.len() {
        sum += (first_list[i] - second_list[i]).abs();
    }
    sum.to_string()
}

pub fn process(input: &str) -> String {
    parser(input)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("11", process(input))
    }
}