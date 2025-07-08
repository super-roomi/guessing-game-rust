// use rand::Rng;
// use std::cmp::Ordering;
use colored::*;
use std::io;

fn main() {
    // println!("Guess the number!");

    // // --generate a random number--
    // let secret_number = rand::thread_rng().gen_range(1..100);

    // println!("The secret number is {}", secret_number);

    // // --loop over the comparison stage for multiple tries--
    // loop {
    //     println!("Please input your guess.");

    //     let mut guess = String::new();

    //     // --get user input for their guess--
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     // --input is taken as string by default, parsing it into an integer
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("you guessed {}", guess);

    //     // --match has arms to compare Less, Greater, and Equal to the value--
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    // let mut a: [i16; 10] = [1, 2, 3, 4, 5, 6, 1, 10, 22, 1];
    // a.sort();

    // for nums in a {
    //     is_even(nums);
    //     is_odd(nums);
    // }
    loop {
        println!(
            "{}",
            "Enter your number please!"
                .bright_magenta()
                .on_white()
                .bold()
        );

        let mut num = String::new();

        // --Take user input (as string ofc ,_,)
        io::stdin()
            .read_line(&mut num)
            .expect("An Error Occurred Reading Line");

        if num.trim() == "exit" {
            println!("Bye bye!");
            break;
        }

        // --Parse into a number so we can modulo and equate
        let num: i32 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("{}", "thats not a number!!".bright_red().bold().italic()),
        };

        if num % 2 == 0 {
            is_even(num);
        } else {
            is_odd(num);
        }
    }
}

fn is_even(number: i32) {
    if number % 2 == 0 {
        println!("{} is even", number);
    }
}

fn is_odd(number: i32) {
    if number % 2 != 0 {
        println!("{} is odd", number)
    }
}
