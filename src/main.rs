use rand::Rng;
use std::cmp::Ordering;
use std::io;

// fn test() {
//     let x = 5;
//     println!("the x is {x}");
    
//     x = 6;
//     println!("next x is {x}");
// }

fn main() {
    let secrete_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Guess the number: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        attempts += 1;
    }
    println!("You tried {attempts} times!")
}
