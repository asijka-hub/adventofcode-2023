use std::fs;

fn line_not_possible(games_str: &str) -> bool {

    for single_game in games_str.split(";") {
        for record in single_game.split(",") {
            let clean = record.strip_prefix(" ").unwrap();
            let (number, color) = clean.split_at(clean.find(" ").unwrap());
            let color = color.strip_prefix(" ").unwrap();
            let number = number.parse::<u32>().unwrap();
            match color {
                "red" => if number > 12 {return true}
                "green" => if number > 13 {return true}
                "blue" => if number > 14 {return true}
                _ => {}
            }
        }
    }

    return false;
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("err");

    let mut final_result = 0;

    for (id, line) in input.lines().enumerate() {
        let (_, games_str) = line.split_at(line.find(":").unwrap());
        let game = games_str.strip_prefix(":").unwrap();
        if !line_not_possible(game) {
            final_result += id + 1;
        }
    }

    println!("{}", final_result);
}
