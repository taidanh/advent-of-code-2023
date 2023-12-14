use std::{cmp::min, char};

fn main() {
    let input = include_str!("./input1.txt");
    let output = gear_ratios(input);
    dbg!(output);
}

fn gear_ratios(s: &str) -> i32 {
    let schema: Vec<Vec<char>> = s.lines()
        .map(|l| { l.chars().collect() })
        .collect();
    let mut sum = 0;

    for r in 0..schema.len() {
        let mut num: String = String::new();
        let mut skip = false;
        for c in 0..schema[0].len() {
            if schema[r][c].is_digit(10) && !skip {
                num.push(schema[r][c].to_owned());
                if search_radius(&schema, r, c) {
                    let mut off = 1;
                    while c+off < schema[0].len() && schema[r][c+off].is_digit(10) {
                        num.push(schema[r][c+off]);
                        off += 1;
                    }
                    sum += num.parse::<i32>().expect("Error reading num");
                    skip = true;
                }
            } else if !schema[r][c].is_digit(10) {
                skip = false;
                if !num.is_empty() {
                    num.clear();
                }
            }
        }
    }
    sum
}

fn search_radius(schema: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let minr = row.saturating_sub(1);
    let maxr = min(row + 1, schema.len() - 1);
    let minc = col.saturating_sub(1);
    let maxc = min(col + 1, schema[0].len() - 1);

    for r in minr..=maxr as usize {
        for c in minc..=maxc as usize {
            if (r, c) == (row, col) { continue; }
            if schema[r][c] != '.' && !schema[r][c].is_digit(10) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();
        assert_eq!(gear_ratios(&l1), 4361);
    }
}
