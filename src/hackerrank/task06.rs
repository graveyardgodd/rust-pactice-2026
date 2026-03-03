use std::io;

fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> String 
{
    if v1 == v2 
    {
        return if x1 == x2 { "YES" } else { "NO" }.to_string();
    }

    if v1 > v2 && (x2 - x1) % (v1 - v2) == 0 
    {
        "YES".into()
    } else 
    {
        "NO".into()
    }
}

fn main() 
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = kangaroo(nums[0], nums[1], nums[2], nums[3]);
    println!("{}", result);
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_sample_yes() {
        assert_eq!(kangaroo(0, 3, 4, 2), "YES");
    }

    #[test]
    fn test_sample_no() {
        assert_eq!(kangaroo(0, 2, 5, 3), "NO");
    }

    #[test]
    fn test_same_position_same_speed() {
        assert_eq!(kangaroo(5, 3, 5, 3), "YES");
    }

    #[test]
    fn test_same_speed_different_position() {
        assert_eq!(kangaroo(0, 2, 5, 2), "NO");
    }

    #[test]
    fn test_negative_case() {
        assert_eq!(kangaroo(2, 1, 10, 2), "NO");
    }
}