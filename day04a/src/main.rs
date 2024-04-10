use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("error");

     println!("{}", input.lines().map(|line| {
        let (win, own) = line.split_at(line.find("|").unwrap());

        let mine: HashSet<u32> = own.split(" ").filter(|&c| c != "|")
            .filter_map(|s| s.parse::<u32>().ok()).collect();
        let wins: HashSet<u32> = win.split(" ").filter(|&c| !c.contains("Card"))
            .filter_map(|s| s.parse::<u32>().ok()).collect();

        mine.iter().fold(0, |acc, x|
            if wins.contains(x) {
                if acc == 0 {1} else {acc * 2}
            } else {acc})
    }).sum::<u32>())
}