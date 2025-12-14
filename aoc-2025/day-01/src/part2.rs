use num::ToPrimitive;
use regex::Regex;

pub fn process(input: &str) -> String {
    // println!("Input 1: {}", input);

    let re = Regex::new(r"(?P<direction>\w)(?P<step>\d+)").unwrap();
    let result = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .fold((50, 0), |(mut point, mut result): (i32, u32), line| {
            let cap = re.captures(line).unwrap();
            let direction = cap.name("direction").map(|m| m.as_str()).unwrap_or("N/A");
            let mut step = cap
                .name("step")
                .map(|m| m.as_str().parse::<i32>().unwrap_or(0))
                .unwrap_or(0);
            if direction.eq("L") {
                if point == 0 {
                    point = 100;
                }
                result += (step / 100) as u32;
                step %= 100;
                point = (point - step) % 100;
                if point < 0 {
                    point += 100;
                    result += 1;
                } else if point == 0 {
                    result += 1;
                }
            } else {
                result += (step / 100) as u32;
                step %= 100;
                point += step;
                if point > 99 {
                    point -= 100;
                    result += 1;
                } else if point == 0 {
                    result += 1;
                }
            }
            // println!("{point}, {result}");
            (point, result)
        })
        .1;

    result.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!("8", process(input))
    }
}
