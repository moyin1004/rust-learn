use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("geuss the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        // 变量默认不可变
        // let data = 10;
        // data += 20; // error
        let mut guess = String::new();
        // 引用默认不可变 可变引用必须使用 mut
        println!("please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
