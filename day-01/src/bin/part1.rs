fn main() {
    let input = include_str!("./input1.txt");
    let output = calibrate(input);
    dbg!(output);
}

fn calibrate(input: &str) -> u32 {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut sum = 0;
    for l in lines {
        if l != "" {
            sum += clean_line(l);
        }
    }
    sum
}

fn clean_line(s: &str) -> u32 {
    let mut digits = Vec::new();
    for c in s.chars() {
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
