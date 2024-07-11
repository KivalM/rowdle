use std::io::BufRead;

use rowdle::Guessable;
extern crate rowdle;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Champion {
    pub name: String,
    pub mana: bool,
    pub location: String,
}

impl Guessable<String> for Champion {
    fn guess(&self, other: &Self) -> rowdle::Guess<Champion, String> {
        let mut result = vec![];

        if self.name == other.name {
            result.push(rowdle::GuessResult::Correct(self.name.clone()));
        } else {
            result.push(rowdle::GuessResult::Incorrect(self.name.clone()));
        }

        if self.mana == other.mana {
            result.push(rowdle::GuessResult::Correct(self.mana.to_string()));
        } else {
            result.push(rowdle::GuessResult::Incorrect(self.mana.to_string()));
        }

        if self.location == other.location {
            result.push(rowdle::GuessResult::Correct(self.location.clone()));
        } else {
            result.push(rowdle::GuessResult::Incorrect(self.location.clone()));
        }

        rowdle::Guess {
            word: self.clone(),
            guess: result,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Wordle!");

    let word = Champion {
        name: "Garen".to_string(),
        mana: false,
        location: "Top".to_string(),
    };

    let word_list = vec![
        Champion {
            name: "Garen".to_string(),
            mana: false,
            location: "Top".to_string(),
        },
        Champion {
            name: "Darius".to_string(),
            mana: false,
            location: "Top".to_string(),
        },
        Champion {
            name: "Vayne".to_string(),
            mana: true,
            location: "Bot".to_string(),
        },
        Champion {
            name: "Zed".to_string(),
            mana: false,
            location: "Mid".to_string(),
        },
        Champion {
            name: "Jinx".to_string(),
            mana: true,
            location: "Bot".to_string(),
        },
    ];

    let mut game = rowdle::Game::new(5, word, word_list.clone());

    while !game.game_over() {
        println!("Enter your guess:");
        let guess = std::io::stdin().lock().lines().next().unwrap()?;

        let guess = word_list.iter().find(|c| c.name == guess);
        match guess {
            Some(guess) => match game.guess(guess.clone()) {
                Ok(res) => {
                    for r in res.guess {
                        print!("{:?} ", r);
                    }
                    println!();
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            },
            None => {
                println!("Champion not found");
            }
        }
    }

    if game.won() {
        println!("Congratulations! You won!");
    } else {
        println!(
            "Sorry, you lost. The word was: {}",
            game.correct_word().name
        );
    }

    Ok(())
}
