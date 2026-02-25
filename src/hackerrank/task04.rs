use std::io::{self, BufRead};

fn solve_grade(g: i32) -> i32 
{
    if g >= 38 && g % 5 >= 3 { g + 5 - g % 5 } else { g }
}

fn main() 
{
    let stdin = io::stdin();
    for line in stdin.lock().lines().skip(1).flatten() 
    {
        if let Ok(g) = line.trim().parse::<i32>() 
        {
            println!("{}", solve_grade(g));
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;
    #[test]
    fn test_logic() 
    {
        assert_eq!(solve_grade(73), 75);
        assert_eq!(solve_grade(67), 67);
        assert_eq!(solve_grade(38), 40);
        assert_eq!(solve_grade(33), 33);
    }
}