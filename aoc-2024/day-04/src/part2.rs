use regex::Regex;

fn find_a(line: &str) -> Vec<usize> {
    let re = Regex::new(r"A").unwrap();
    re.find_iter(line).map(|m| m.start()).collect()
}

fn find_checkpoint(a_point: &(usize, usize), len: usize) -> Option<Vec<(usize, usize)>> {
    let (l, i) = *a_point;
    if l == 0 || l == len - 1 {
        return None;
    }

    if i == 0 || i == len - 1 {
        return None;
    }

    let result = vec![
        (l - 1, i - 1),
        (l + 1, i + 1),
        (l - 1, i + 1),
        (l + 1, i - 1),
    ];

    Some(result)
}

fn check_xmas(checkpoints: &[(usize, usize)], data: &[String]) -> bool {
    let re = Regex::new(r"(?:MS|SM)");
    let (l0, i0) = checkpoints[0];
    let (l1, i1) = checkpoints[1];
    let (l2, i2) = checkpoints[2];
    let (l3, i3) = checkpoints[3];

    // Combine chars to string
    let chars = [
        data[l0].chars().nth(i0).unwrap_or('X'),
        data[l1].chars().nth(i1).unwrap_or('X'),
    ];
    let s1: String = chars.iter().collect();

    let chars = [
        data[l2].chars().nth(i2).unwrap_or('X'),
        data[l3].chars().nth(i3).unwrap_or('X'),
    ];
    let s2: String = chars.iter().collect();

    // println!("{} {}", s1, s2);

    // Using regex to check if s1 and s2 match
    let match_s1 = re.as_ref().unwrap().is_match(&s1);
    let match_s2 = re.as_ref().unwrap().is_match(&s2);

    match_s1 && match_s2
}

fn parser(input: &str) -> Vec<String> {
    let re = Regex::new(r"(?:\w+$)").unwrap();
    let data = input
        .lines()
        .filter(|line| re.is_match(line))
        .map(|line| line.to_string())
        .collect();
    data
}

pub fn process(input: &str) -> String {
    let data = parser(input);
    let len = data.len();
    let a_position = data
        .iter()
        .enumerate()
        .flat_map(|(l, s)| {
            find_a(s)
                .iter()
                .map(|i| (l, *i))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();
    // println!("{:?}", a_position);
    let result = a_position.iter().fold(0u32, |acc, a_point| {
        find_checkpoint(a_point, len)
            .iter()
            .filter(|c| {
                // println!("{:?} {:?}", a_point, c);
                check_xmas(c, &data)
            })
            .count() as u32
            + acc
    });

    result.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("9", process(input))
    }
}
