use std::io;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i32, b: i32) -> i32 {
    a / gcd(a, b) * b
}

fn get_total_x(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let l = a.into_iter().reduce(|x, y| lcm(x, y)).unwrap();
    let g = b.into_iter().reduce(|x, y| gcd(x, y)).unwrap();

    let mut count = 0;
    let mut multiple = l;

    while multiple <= g {
        if g % multiple == 0 {
            count += 1;
        }
        multiple += l;
    }

    count
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nm: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let a: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}", get_total_x(a, b));
}