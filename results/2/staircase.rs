use std::io;

fn main() 
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    for i in 1..=n 
    {
        println!("{}{}", " ".repeat(n - i), "#".repeat(i));
    }
}