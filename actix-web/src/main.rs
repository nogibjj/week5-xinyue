use std::io::{self, Write};
use rand::seq::SliceRandom;

fn main() {
    let words = vec!["apple", "banana", "cherry", "date", "elderberry", "fig"];
    let mut rng = rand::thread_rng();
    let word = words.choose(&mut rng).unwrap();
    let mut letters: Vec<Option<char>> = vec![None; word.len()];
    let mut remaining_guesses = 6;

    loop {
        println!("Word: {:?}", letters);
        println!("Guesses remaining: {}", remaining_guesses);

        if !letters.contains(&None) {
            println!("Congratulations, you win!");
            break;
        }

        if remaining_guesses == 0 {
            println!("Sorry, you lose. The word was {}.", word);
            break;
        }

        print!("Guess a letter: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess = guess.chars().next().unwrap();

        if word.contains(guess) {
            for (i, c) in word.chars().enumerate() {
                if c == guess {
                    letters[i] = Some(c);
                }
            }
        } else {
            remaining_guesses -= 1;
        }
    }
}
