use std::cmp::Ordering;
use std::fs;

#[derive(Debug)]
struct Hand {
    cards: String,
    bet: u16,
}

// A A B B C
// A B C C C

impl Hand {
    fn my_type(&self) -> u8 {
        // println!("dupa");
        // for c in self.cards.chars() {
        //     println!("{}", c);
        // }
        let mut vec: Vec<char> = self.clone().cards.chars().collect();
        vec.sort();
        println!("{:#?}", vec);
        let mut rank: u8 = 0;
        let mut curr_rank: u8 = 1;
        for i in 1..5 {
            println!("{}", vec[i]);
            if vec[i - 1] == vec[i] {
                curr_rank = curr_rank + 1;
            } else {
                rank = rank.max(curr_rank);
                curr_rank = 1;
            }
        }
        println!("{}", rank);
        rank
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
        self.cards.cmp(&other.cards)
    }
}

fn main() {
    let input: String = fs::read_to_string("example.txt").unwrap();

    let hand: Hand = Hand{cards: "ABABC".parse().unwrap(), bet: 27};

    println!("{:?}", hand);

    hand.my_type();

    // let mut hands: Vec<Hand> = input.lines().map(|l| {
    //     let mut iter = l.split_whitespace();
    //     Hand {
    //         cards: iter.next().unwrap().to_string(),
    //         bet: iter.next().unwrap().parse::<u16>().unwrap(),
    //     }
    // }).collect();
    //
    // hands.sort();
    //
    // let mut res: u64 = 0;
    //
    // for (i, Hand {cards, bet}) in hands.iter().enumerate() {
    //     res += (*bet as u64) * (i as u64);
    // }
    //
    // // println!("{:#?}", hands);
    // println!("{}", res);
}
