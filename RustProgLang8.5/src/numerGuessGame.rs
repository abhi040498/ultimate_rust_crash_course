use rand::Rng;
use std::io::stdin;
fn main()
{
    let number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Enter the number you guessed: ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trin_end().parse::<i64>();
                match parsed{
                    Ok(guess) => {
                    
                        if guess < 1|| guess > 100 {
                            println!("You are guessing and put of range value.");
                        } else if guess < number {
                            println!("You are guessing a low value.");
                        } else if guess > number {
                            println!("You are guessing and put of range value.");
                        }
                    },
                    Err(e) => {
                        println!("Could not red your input {} , Try again please", e);
                    }
                }
            }, 
            Err(_) => continue,
        }
    }
}