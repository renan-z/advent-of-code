use regex::Regex;

pub fn parser(input: &str) -> Vec<Vec<u32>> {
    let re = Regex::new(r"\d+").unwrap();
    let mut data: Vec<Vec<u32>> = Vec::new();
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect()
        })
        .for_each(|v| data.push(v));
    data
}

pub fn process(input: &str) -> String {
    let data = parser(input);
    let mut result = 0;
    for report in data {
        // Check if levels (items) are all increasing or all decreasing
        if report.windows(2).all(|w| w[0] < w[1]) || report.windows(2).all(|w| w[0] > w[1]) {
            // Check if the diff of each level <1 and >3
            if report.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3) {
                // println!("{:?}", report);
                result += 1;
            }
        }
    }
    result.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input))
    }
}
