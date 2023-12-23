use std::u32;

fn main() {
    let input = include_str!("./input1.txt");
    let output = scratchcards(input);
    dbg!(output);
}

fn scratchcards(s: &str) -> i32 {
    let mut total: i32 = 0;
    for line in s.lines() {
        let x: Vec<&str> = line.split('|').collect();
        let winning_nums = &x[0]
            .split_whitespace()
            .collect::<Vec<&str>>()[2..]; // ignore 'Card x:'
        let winning_nums: Vec<i32> = winning_nums.iter()
            .map(|num| {
                num.parse::<i32>().expect("Unexpected non-digit found")
            }).collect();
        let found_nums = &x[1]
            .split_whitespace()
            .collect::<Vec<&str>>();
        let found_nums: Vec<i32> = found_nums.iter()
            .map(|num| {
                num.parse::<i32>().expect("Unexpected non-digit found")
            }).collect();
        let mut wins = 0;
        for num in winning_nums {
            // check for dupes
            if found_nums.contains(&num) {
                wins = if wins == 0 { 1 } else { wins * 2 };
            }
        }
        total += wins;
        println!("wins: {}", wins);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();
        assert_eq!(scratchcards(&l1), 13);
    }
}
