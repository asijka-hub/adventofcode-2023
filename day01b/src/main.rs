use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path)
        .expect("error reading a file");

    let result =  contents.lines().map(
        |line| {
            let ones = (line.replace("one", "1").find("one"), 1);
        }
    )

    println!("result: {}", result);
}
