use std::io;

fn build_staircase(n: usize) -> Vec<String> 
{
    let mut result = Vec::new();
    for i in 1..=n 
    {
        result.push(format!("{:>width$}", "#".repeat(i), width = n));
    }
    result
}

fn staircase(n: i32) 
{
    let lines = build_staircase(n as usize);
    for line in lines 
    {
        println!("{}", line);
    }
}

#[allow(dead_code)]
fn main() {
    let mut input_string = String::new();
    if io::stdin().read_line(&mut input_string).is_ok() 
    {
        if let Ok(n) = input_string.trim().parse::<i32>() 
        {
            staircase(n);
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_staircase_size_6() 
    {
        let expected = vec!
        [
            "     #",
            "    ##",
            "   ###",
            "  ####",
            " #####",
            "######"
        ];
        assert_eq!(build_staircase(6), expected);
    }

    #[test]
    fn test_staircase_size_1() 
    {
        let expected = vec!["#"];
        assert_eq!(build_staircase(1), expected);
    }
}