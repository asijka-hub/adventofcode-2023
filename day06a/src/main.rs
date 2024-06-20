use std::fs;

fn solve_quadratic(a: f64, b: f64, c: f64) -> (Option<f64>, Option<f64>) {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        (None, None)
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let t1 = (-b + sqrt_discriminant) / (2.0 * a);
        let t2 = (-b - sqrt_discriminant) / (2.0 * a);
        (Some(t1), Some(t2))
    }
}

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let time_str: String = input.lines().rev().skip(1).collect();
    let distance_str: String = input.lines().skip(1).collect();

    let time: Vec<u64> = time_str.split_whitespace().skip(1).map(|s| s.parse::<u64>().unwrap()).collect();
    let distance: Vec<u64> = distance_str.split_whitespace().skip(1).map(|s| s.parse::<u64>().unwrap()).collect();

    let mut res: u64 = 1;

    for (i, t) in time.iter().enumerate() {
        let d = distance[i];
        let (t1, t2) = solve_quadratic(-1f64, *t as f64, -(d as f64));

        let b = t1.clone().unwrap().ceil() as u64;
        let e = t2.clone().unwrap().floor() as u64;

        let mut n: u64 = e - b + 1;

        if t1.unwrap() % 10f64 == 0f64 {
            n = n - 1;
        }

        if t2.unwrap() % 10f64 == 0f64 {
            n = n - 1;
        }

        res = res * n;
    }

    println!("{}", res);

}
