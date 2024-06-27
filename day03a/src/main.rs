use std::fs;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Symbol {
    pos: Position,
}

#[derive(Debug)]
struct Number {
    value: i64,
    pos: Vec<Position>,
}

fn next_to_each_other(n: &Position, s: &Position) -> bool {
    let mut res: bool = false;

    if n.x == s.x && n.y == s.y - 1 { res = true; }
    if n.x == s.x && n.y == s.y + 1 { res = true; }
    if n.x == s.x - 1 && n.y == s.y { res = true; }
    if n.x == s.x + 1 && n.y == s.y { res = true; }

    if n.x == s.x + 1 && n.y == s.y - 1 { res = true; }
    if n.x == s.x - 1 && n.y == s.y - 1 { res = true; }
    if n.x == s.x + 1 && n.y == s.y + 1 { res = true; }
    if n.x == s.x - 1 && n.y == s.y + 1 { res = true; }

    res
}

impl Number {
    fn next_to_symbol(&self, positions: &Vec<Symbol>) -> bool {
        let mut res: bool = false;

        for p in &self.pos {
            for pos in positions {
                if next_to_each_other(p, &pos.pos) {
                    res = true;
                }
            }
        }

        res
    }
}

fn part1(numbers: &Vec<Number>, positions: &Vec<Symbol>) -> i64 {
    numbers.iter().filter(|n| n.next_to_symbol(&positions))
        .map(|n| { n.value }).sum()
}

const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let mut symbols: Vec<Symbol> = Vec::new();
    let mut numbers: Vec<Number> = Vec::new();

    let mut curr_number: Vec<char> = Vec::new();
    let mut curr_positions: Vec<Position> = Vec::new();

    let width: i64 = input.clone().lines().next().unwrap().len() as i64;

    for (x, l) in input.lines().enumerate() {
        for (y, c) in l.chars().enumerate() {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    curr_number.push(c);
                    curr_positions.push(Position { x: x as i64, y: y as i64 });
                }
                '.' => {
                    if curr_number.len() > 0 {
                        let s: String = curr_number.iter().collect();
                        // println!("{s}");
                        let parsed: i64 = s.parse::<i64>().unwrap();
                        numbers.push(Number { value: parsed, pos: curr_positions });
                        curr_number.clear();
                        curr_positions = Vec::new();
                    }
                }
                _ => {
                    symbols.push(Symbol { pos: Position { x: x as i64, y: y as i64 } });
                    if curr_number.len() > 0 {
                        let s: String = curr_number.iter().collect();
                        // println!("{s}");
                        let parsed: i64 = s.parse::<i64>().unwrap();
                        numbers.push(Number { value: parsed, pos: curr_positions });
                        curr_number.clear();
                        curr_positions = Vec::new();
                    }
                }
            }
        }

        if curr_number.len() > 0 {
            let s: String = curr_number.iter().collect();
            // println!("{s}");
            let parsed: i64 = s.parse::<i64>().unwrap();
            numbers.push(Number { value: parsed, pos: curr_positions });
            curr_number.clear();
            curr_positions = Vec::new();
        }
    }

    println!("{}", { part1(&numbers, &symbols) });
}
