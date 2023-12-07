fn main() {
    let input = include_str!("./input1.txt");
    let output = calibrate_part2(input);
    dbg!(output);
}

fn calibrate_part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut sum = 0;
    for l in lines {
        if l != "" {
            sum += clean_line(l);
        }
    }
    sum
}

//                                (index, word len)
fn check_num(s: &str, i usize) -> (u32, u32) {
    let nums = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    for (i, n) in nums.iter().enumerate() {
        if s.starts_with(n) {
            return (i, nums[i].len());
        }
    }
}

fn clean_line(s: &str) -> u32 {
    // println!("s: {:?}", s);
    let mut digits = Vec::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_digit(10) {
            digits.push(c);
        }
    }
    format!("{}{}", &digits[0], &digits[digits.len() - 1])
        .parse()
        .expect("Not a valid integer")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = "1abc2".to_string();
        assert_eq!(clean_line(&l1), 12);
    }
}
