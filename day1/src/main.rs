use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("error reading a file");

    let result =  contents.split_whitespace().map(
        |word| {
            let first = word.chars().find(|x| x.is_digit(10)).unwrap().to_digit(10).unwrap();
            let second = word.chars().rev().find(|x| x.is_digit(10)).unwrap().to_digit(10).unwrap();
            return 10 * first + second;
        } as u32
    ).sum::<u32>();

    println!("result: {}", result);
}
