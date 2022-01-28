use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}",
                   value);
        }
        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).
            expect("Failed to Read Guess");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => Guess::new(num).value(),
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
