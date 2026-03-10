use std::io;

fn migratory_birds(arr: &[i32]) -> i32 {
    let mut freq = [0; 6];
    for &id in arr {
        freq[id as usize] += 1;
    }

    let mut max = 0;
    let mut result = 1;

    for i in 1..=5 {
        if freq[i] > max {
            max = freq[i];
            result = i as i32;
        }
    }

    result
}

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let arr: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = migratory_birds(&arr);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_0() {
        let arr = vec![1, 4, 4, 4, 5, 3];
        assert_eq!(migratory_birds(&arr), 4);
    }

    #[test]
    fn sample_1() {
        let arr = vec![1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4];
        assert_eq!(migratory_birds(&arr), 3);
    }
}