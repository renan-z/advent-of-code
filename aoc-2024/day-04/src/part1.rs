use regex::Regex;

fn rotate_90_clockwise(matrix: &[String]) -> Vec<String> {
    let cols = matrix[0].len();

    (0..cols)
        .map(|col| {
            matrix
                .iter()
                .rev()
                .map(|row| row.chars().nth(col).unwrap())
                .collect()
        })
        .collect()
}

fn rotate_45_degrees_anti(matrix: &[String]) -> Vec<String> {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let chars: Vec<Vec<char>> = matrix.iter().map(|row| row.chars().collect()).collect();

    let mut result = Vec::new();

    // Extract anti-diagonals from top-right to bottom-left
    // Start from top row, right to left
    for start_col in (0..cols).rev() {
        let mut diagonal = String::new();
        let mut row = 0;
        let mut col = start_col;

        while row < rows && col < cols {
            diagonal.push(chars[row][col]);
            if col == 0 {
                break;
            }
            row += 1;
            col -= 1;
        }
        result.push(diagonal);
    }

    // Continue from left column, top to bottom
    for start_row in 1..rows {
        let mut diagonal = String::new();
        let mut row = start_row;
        let mut col = cols - 1;

        while row < rows {
            diagonal.push(chars[row][col]);
            if col == 0 {
                break;
            }
            row += 1;
            col -= 1;
        }
        result.push(diagonal);
    }

    result
}

fn find_xmas(input: &str) -> u32 {
    let re = Regex::new(r"XMAS").unwrap();
    re.find_iter(input).count() as u32
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
    let mut data = parser(input);
    let mut result = data.iter().map(|line| find_xmas(line)).sum::<u32>();

    let tmp_data = rotate_45_degrees_anti(&data);
    result += tmp_data.iter().map(|line| find_xmas(line)).sum::<u32>();

    let tmp_data = rotate_90_clockwise(&data);
    result += tmp_data.iter().map(|line| find_xmas(line)).sum::<u32>();
    data = tmp_data;

    let tmp_data = rotate_45_degrees_anti(&data);
    result += tmp_data.iter().map(|line| find_xmas(line)).sum::<u32>();

    let tmp_data = rotate_90_clockwise(&data);
    result += tmp_data.iter().map(|line| find_xmas(line)).sum::<u32>();
    data = tmp_data;

    let tmp_data = rotate_45_degrees_anti(&data);
    result += tmp_data.iter().map(|line| find_xmas(line)).sum::<u32>();

    let tmp_data = rotate_90_clockwise(&data);
    result += tmp_data.iter().map(|line| find_xmas(line)).sum::<u32>();
    data = tmp_data;

    let tmp_data = rotate_45_degrees_anti(&data);
    result += tmp_data.iter().map(|line| find_xmas(line)).sum::<u32>();

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
        assert_eq!("18", process(input))
    }
}
