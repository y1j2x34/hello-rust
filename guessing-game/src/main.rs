use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    let mut times = 0;
    let result = loop {
        times = times + 1;
        let mut input = String::new();
        read_line(&mut input);
        let input = match parse_to_usize(input) {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                break secret_number;
            }
        }
    };
    println!("You won, the number is {},  and you guessed {} times in total.", result, times);
}
fn read_line(input: &mut String) -> usize {
    io::stdin().read_line(input).expect("Faild to read line")
}
fn parse_to_usize(input: String) -> Result<usize, std::num::ParseIntError> {
    input.trim().parse::<usize>()
}