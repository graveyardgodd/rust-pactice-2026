use std::io;

fn breaking_records(scores: Vec<i32>) -> (i32, i32) {
    let mut min = scores[0];
    let mut max = scores[0];
    let mut min_count = 0;
    let mut max_count = 0;

    for &score in scores.iter().skip(1) {
        if score > max {
            max = score;
            max_count += 1;
        } else if score < min {
            min = score;
            min_count += 1;
        }
    }

    (max_count, min_count)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let scores: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let (max_count, min_count) = breaking_records(scores);
    println!("{} {}", max_count, min_count);
}