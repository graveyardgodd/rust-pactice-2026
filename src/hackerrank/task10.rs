use std::collections::HashMap;
use std::io;

fn sock_merchant(ar: Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for color in ar {
        *map.entry(color).or_insert(0) += 1;
    }

    map.values().map(|x| x / 2).sum()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let _n: usize = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    let ar: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = sock_merchant(ar);
    println!("{}", result);
}