//use std::io;c
use std::io::{self, Write};
fn main() {
         println!("Evil Guessing Game (1..=15) — easy test mode");
    println!("Type Q/q, 'exit', or 'quit' to stop the game.");
    println!("(The program adapts its 'secret' to maximize your guesses.)");
    let mut low: u32 = 1;
    let mut high: u32 = 15;
    let mut guesses: u32 = 0;

loop {
        print!("Please input your guess (1-15) or Q to quit: ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            println!("Couldn't read input. Try again.");
            continue;
        }
        let input = line.trim();

        // allow quitting
        if input.eq_ignore_ascii_case("q")
            || input.eq_ignore_ascii_case("exit")
            || input.eq_ignore_ascii_case("quit")
        {
            println!("Goodbye — game ended after {guesses} guesses.");
            break;
        }

        // parse guess
        let g: u32 = match input.parse() {
            Ok(n) if (1..=15).contains(&n) => n,
            _ => {
                println!("Enter a number between 1 and 15, or Q to quit.");
                continue;
            }
        };

        guesses += 1;

        // contradicts earlier hints?
        if g < low || g > high {
            println!("That guess contradicts earlier hints. Try another.");
            continue;
        }

        // forced single value and guessed -> win
        if low == high && g == low {
            println!("You win! (Forced in {guesses} guesses.)");
            break;
        }

        // size if we keep lower-range [low..=g-1] vs upper-range [g+1..=high]
        let size_low = if g > low { g - low } else { 0 };     // count of values < g in range
        let size_high = if g < high { high - g } else { 0 };  // count of values > g in range

        // keep the larger side (tie -> keep higher side)
        if size_high >= size_low && size_high > 0 {
            low = g + 1;
            println!("Too small!");
        } else if size_low > 0 {
            high = g - 1;
            println!("Too big!");
        } else {
            // both zero implies only possible value was g, but we already handled the forced win case
            println!("Try another number.");
        }
    }
}