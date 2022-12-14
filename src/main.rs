use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess : u32 = guess.trim().parse().expect("Please type a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is : {secret_number}");

    println!("Your number was : {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!")
    };

}