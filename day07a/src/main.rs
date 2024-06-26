use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Hand {
    cards: String,
    bet: u16,
}

fn card_value(c: char) -> u8 {
    match c {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
         _  => 0
    }
}

impl Hand {
    fn my_type(&self) -> u8 {
        let mut vec: Vec<char> = self.clone().cards.chars().collect();
        let mut set = HashSet::new();
        for c in vec.clone() {
            set.insert(c);
        }

        let mut ranked: Vec<u8> = set.iter().map(|e| {
            let how_many: u8 = vec.iter().map(|c| if c == e {1} else {0}).sum();
            how_many
        }).collect();

        ranked.sort();

        if ranked.len() == 1 {
            return 50
        }

        ranked[ranked.len() - 1] * 10 + ranked[ranked.len() - 2]
    }

    fn compare_each(&self, other: &Hand) -> Ordering {
        let mut vec1: Vec<char> = self.clone().cards.chars().collect();
        let mut vec2: Vec<char> = other.clone().cards.chars().collect();

        for (i, c) in vec1.iter().enumerate() {
            if card_value(*c) == card_value(vec2[i]) {
                continue
            } else {
                return card_value(*c).cmp(&card_value(vec2[i]))
            }
        }

        Ordering::Equal
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards.eq(&other.cards)
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.my_type().cmp(&other.my_type()) == Equal {
            self.compare_each(other)
        } else {
            self.my_type().cmp(&other.my_type())
        }
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let mut hands: Vec<Hand> = input.lines().map(|l| {
        let mut iter = l.split_whitespace();
        Hand {
            cards: iter.next().unwrap().to_string(),
            bet: iter.next().unwrap().parse::<u16>().unwrap(),
        }
    }).collect();

    hands.sort();

    let mut res: u64 = 0;

    for (i, Hand {cards, bet}) in hands.iter().enumerate() {
        res += (*bet as u64) * ((i + 1) as u64);
    }

    println!("{}", res);
}
