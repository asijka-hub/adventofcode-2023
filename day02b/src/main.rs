use std::{cmp, fs};

fn power_set(games_str: &str) -> u32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;


    for single_game in games_str.split(";") {
        for record in single_game.split(",") {
            let clean = record.strip_prefix(" ").unwrap();
            let (number, color) = clean.split_at(clean.find(" ").unwrap());
            let color = color.strip_prefix(" ").unwrap();
            let number = number.parse::<u32>().unwrap();
            match color {
                "red" => {min_red = cmp::max(number, min_red);}
                "green" => {min_green = cmp::max(number, min_green);}
                "blue" => {min_blue = cmp::max(number, min_blue);}
                _ => {}
            }
        }
    }

    return min_red * min_green * min_blue;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("err");

    let mut final_result = 0;

    for (id, line) in input.lines().enumerate() {
        let (_, games_str) = line.split_at(line.find(":").unwrap());
        let game = games_str.strip_prefix(":").unwrap();
        final_result += power_set(game);
    }

    println!("{}", final_result);
}
