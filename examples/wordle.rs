use std::io::BufRead;

use random_word::Lang;

extern crate rowdle;

fn gen_words(n: usize) -> (String, Vec<String>) {
    let words = random_word::all_len(n, Lang::En).unwrap();
    // to vec of strings
    let words = words.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let random = random_word::gen_len(n, Lang::En).unwrap().to_string();

    (random, words)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Wordle!");
    println!("How long should the word be?");
    let n: usize = std::io::stdin().lock().lines().next().unwrap()?.parse()?;
    let (word, word_list) = gen_words(n);

    let mut game = rowdle::Game::new(5, word, word_list);

    while !game.game_over() {
        println!("Enter your guess:");
        let guess = std::io::stdin().lock().lines().next().unwrap()?;

        match game.guess(guess.clone()) {
            Ok(res) => {
                for r in res.guess {
                    print!("{:?} ", r);
                }
                println!();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    if game.won() {
        println!("Congratulations! You won!");
    } else {
        println!("Sorry, you lost. The word was: {}", game.correct_word());
    }

    Ok(())
}
