use std::io;
use rand::prelude::*;
use std::cmp::Ordering;
fn main()
{
    println!("Guess the num!");
    let secret_num=rand::thread_rng().gen_range(1..=100);
loop 
{
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Fail to read");
    println!("Your guessed:{}", guess);
    let guess:u32 = match guess.trim().parse()
            {
                Ok(num) => num,
                Err(_) => continue,
            };
    match guess.cmp(&secret_num)
    {
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => 
        {
            println!("You win");
            break;
        },
        Ordering::Less => println!("Too small"),
    }
}
}