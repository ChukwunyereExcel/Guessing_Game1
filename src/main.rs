use std::cmp::Ordering;

use std::io;
// use rand::Rng;
// explaining the use of the rand crate and the io module from the standard library
// rand is a module that provides random number generation functionality, while io is used for handling input and output operations.
// Rng is a trait that defines methods for generating random numbers, and we import it to use its functionality in our code.
//gen_range is a method provided by the Rng trait that generates a random number within a specified range. In this case, we use it to generate a random number between 1 and 100 (inclusive).
/*
so the structure of rand is as follows might not be the correct code but yeah:
mod rand* {

    pub trait Rng* {
        thread_rng*() -> ThreadRng;
            fn gen_range*(&mut self, range: Range) -> T;
            // other methods...
    }
    // other modules and types...
}

to use the gen_range method, we need to import the Rng trait and call it on a random number generator instance, such as thread_rng(), which provides a thread-local random number generator.
like so: rand*::Rng*::thread_rng*().gen_range*(1..=100);
*/

fn main() {
    println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1..=100);
    // because we are using the rand = 0.10 crate, we can use the rand::random_range function to generate a random number in the specified range, the above syntax did not work, so the below syntax is used instead.
    let secret_number = rand::random_range(1..=100);
    println!("The secret number is: {}", secret_number); // For testing purposes only

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // a static method used to create a new, empty, and growable UTF-8 encoded string.
        // You can now grow the string
        // my_string.push_str("Hello, Rust!");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        /*
        Appends Data: The read_line method appends the data to the provided String. If you call it multiple times in a loop, make sure to call input.clear() beforehand to empty the buffer.
        Returns a Result: It returns an io::Result<usize>. On success, the usize represents the total number of bytes read, including the trailing newline character.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } // This line is unreachable because all possible cases of Ordering are covered above. However, Rust requires a catch-all case for match expressions, so we include it here to satisfy the compiler.
        }
    }
    /* Shift + Alt + A
           A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern. Rust takes the value given to match and looks through each arm’s pattern in turn. Patterns and the match construct are powerful Rust features: They let you express a variety of situations your code might encounter, and they make sure you handle them all. These features will be covered in detail in Chapter 6 and Chapter 19, respectively.

    Let’s walk through an example with the match expression we use here. Say that the user has guessed 50 and the randomly generated secret number this time is 38.

    When the code compares 50 to 38, the cmp method will return Ordering::Greater because 50 is greater than 38. The match expression gets the Ordering::Greater value and starts checking each arm’s pattern. It looks at the first arm’s pattern, Ordering::Less, and sees that the value Ordering::Greater does not match Ordering::Less, so it ignores the code in that arm and moves to the next arm. The next arm’s pattern is Ordering::Greater, which does match Ordering::Greater! The associated code in that arm will execute and print Too big! to the screen. The match expression ends after the first successful match, so it won’t look at the last arm in this scenario.
        */

        // syntax error mostly, where you see '//' you wiill where we made a mistake, and where you see '/*' you will see the correct syntax, and where you see '*/' you will see the end of the correct syntax.
}
// Ctrl + ` to open the terminal in VS Code, and then run the command cargo run to compile and run the program. You can also use the shortcut Ctrl + Shift + B to build the project, and then run the executable from the terminal.