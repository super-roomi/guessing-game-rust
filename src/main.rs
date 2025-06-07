use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // --generate a random number--
    let secret_number = rand::thread_rng().gen_range(1..100);

    println!("The secret number is {}", secret_number);

    // --loop over the comparison stage for multiple tries--
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // --get user input for their guess--
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // --input is taken as string by default, parsing it into an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {}", guess);

        // --match has arms to compare Less, Greater, and Equal to the value--
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
