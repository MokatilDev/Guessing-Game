use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("{}","Guess the number!".on_blue());
    // println!("Made by MokatilDev");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("{}","=============================".blue());

        // println!("Secret number {}", secret_number);
        println!("- Please input your guess 🔢.");

        let mut guess = String::new(); 
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        // println!("- You guessed {}", guess);kk

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
        };
    }
}
nm,