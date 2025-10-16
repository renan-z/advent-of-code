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

pub fn is_differ_less_than_3(report: &[u32]) -> bool {
    report.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3)
}

pub fn is_increasing(report: &[u32]) -> bool {
    report.windows(2).all(|w| w[0] < w[1])
}

pub fn is_decreasing(report: &[u32]) -> bool {
    report.windows(2).all(|w| w[0] > w[1])
}

pub fn is_safe(report: &[u32]) -> bool {
    (is_increasing(report) || is_decreasing(report)) && is_differ_less_than_3(report)
}

pub fn is_safe_with_dampener(report: &[u32]) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut new_report = report.to_vec();
        new_report.remove(i);

        if is_safe(&new_report) {
            return true;
        }
    }
    false
}

pub fn process(input: &str) -> String {
    let data = parser(input);
    let result = data
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count();
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
        assert_eq!("4", process(input))
    }
}
