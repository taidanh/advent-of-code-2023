use std::cmp;

fn main() {
    let input = include_str!("./input1.txt");
    let output = cube_conundrum(input);
    dbg!(output);
}

fn cube_conundrum(text: &str) -> i32 {
    let games: Vec<&str> = text.split('\n').collect();
    let mut possible_games: Vec<i32> = Vec::new();
    println!("last: `{}`", games[games.len()-1]);
    for &game in &games {
        let line: Vec<&str> = game.split(' ').collect();
        if line == [""] {
            break;
        }
        let id: i32 = line[1][..line[1].len() - 1]
            .to_string().parse().expect("Id isn't a digit");
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        for pair in line[2..].chunks(2) {
            if pair[1].starts_with("r") {
                red = cmp::max(red, pair[0].parse::<i32>().unwrap())
            } else
            if pair[1].starts_with("g") {
                green = cmp::max(green, pair[0].parse::<i32>().unwrap())
            } else
            if pair[1].starts_with("b") {
                blue = cmp::max(blue, pair[0].parse::<i32>().unwrap())
            }
        }
        if red <= 12 && green <= 13 && blue <= 14 {
            possible_games.push(id);
        }
    }
    println!("num: {}\ngames: {:?}", possible_games.len(), possible_games);
    possible_games.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let l1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        assert_eq!(cube_conundrum(&l1), 8);
    }
}
