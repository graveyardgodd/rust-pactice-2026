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

#[allow(dead_code)]
fn main()
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(n) = input.trim().parse::<usize>() 
    {
        let lines = build_staircase(n);
        for line in lines 
        {
            println!("{}", line);
        }
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_staircase_size_3() 
    {
        let expected = vec!
        [
            "  #",
            " ##",
            "###"
        ];
        assert_eq!(build_staircase(3), expected);
    }

    #[test]
    fn test_staircase_size_1() 
    {
        let expected = vec!["#"];
        assert_eq!(build_staircase(1), expected);
    }
}