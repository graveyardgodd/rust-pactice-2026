use std::io;

pub fn count_apples_and_oranges
(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: Vec<i32>,
    oranges: Vec<i32>,
) -> (usize, usize) 
{
    let apple_count = apples
        .iter()
        .filter(|&&distance| 
        {
            let position = a + distance;
            position >= s && position <= t
        })
        .count();

    let orange_count = oranges
        .iter()
        .filter(|&&distance|
        {
            let position = b + distance;
            position >= s && position <= t
        })
        .count();

    (apple_count, orange_count)
}

fn main() 
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let first_line: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let s = first_line[0];
    let t = first_line[1];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let second_line: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let a = second_line[0];
    let b = second_line[1];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let third_line: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let m = third_line[0] as usize;
    let n = third_line[1] as usize;

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let apples: Vec<i32> = input
        .split_whitespace()
        .take(m)
        .map(|x| x.parse().unwrap())
        .collect();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let oranges: Vec<i32> = input
        .split_whitespace()
        .take(n)
        .map(|x| x.parse().unwrap())
        .collect();

    let (apple_count, orange_count) =
        count_apples_and_oranges(s, t, a, b, apples, oranges);

    println!("{}", apple_count);
    println!("{}", orange_count);
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn sample_input_test() 
    {
        let s = 7;
        let t = 11;
        let a = 5;
        let b = 15;

        let apples = vec![-2, 2, 1];
        let oranges = vec![5, -6];

        let result = count_apples_and_oranges(s, t, a, b, apples, oranges);

        assert_eq!(result, (1, 1));
    }

    #[test]
    fn no_fruit_on_house() 
    {
        let result = count_apples_and_oranges
        (
            10,
            20,
            0,
            30,
            vec![1, 2, 3],
            vec![-1, -2, -3],
        );

        assert_eq!(result, (0, 0));
    }
}