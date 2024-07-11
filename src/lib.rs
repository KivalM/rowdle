use std::fmt::Debug;

pub use guesser::{Guess, GuessResult, Guessable};

pub mod error;
pub mod guesser;

pub struct Game<T: PartialEq + Clone, G: Guessable<T> + Default + Debug> {
    max_tries: u8,
    correct_word: G,
    word_list: Vec<G>,
    guesses: Vec<Guess<G, T>>,
}

impl<T: PartialEq + Clone, G: Guessable<T> + Default + Debug> Game<T, G> {
    pub fn new(max_tries: u8, correct_word: G, word_list: Vec<G>) -> Self {
        Self {
            max_tries,
            correct_word,
            word_list,
            guesses: vec![],
        }
    }

    pub fn guess(&mut self, word: G) -> Result<Guess<G, T>, error::WordleError<G>> {
        if self.max_tries == 0 {
            return Err(error::WordleError::MaxTriesExceeded);
        }

        if !self.word_list.contains(&word) {
            return Err(error::WordleError::InvalidWord(word));
        }

        if self.is_word_guessed(&word) {
            return Err(error::WordleError::WordAlreadyGuessed(word));
        }

        let res = word.guess(&self.correct_word);
        self.guesses.push(res);
        Ok(self.guesses.last().unwrap().clone())
    }

    pub fn is_word_guessed(&self, word: &G) -> bool {
        self.guesses.iter().any(|g| g.word == *word)
    }

    pub fn won(&self) -> bool {
        self.guesses.iter().any(|g| g.word == self.correct_word)
    }

    pub fn lost(&self) -> bool {
        self.guesses.len() == self.max_tries.into() && !self.won()
    }

    pub fn game_over(&self) -> bool {
        self.won() || self.lost()
    }

    pub fn correct_word(&self) -> &G {
        &self.correct_word
    }

    pub fn board(&self, pad: Option<u32>, buffer: Option<Guess<G, T>>) -> Vec<Guess<G, T>> {
        let mut guesses = self.guesses.clone();

        if let Some(buffer) = buffer {
            guesses.push(buffer);
        }

        if let Some(pad) = pad {
            let num_cell = self.max_tries as u32 - guesses.len() as u32;
            let n = pad.min(num_cell);

            for _ in 0..n {
                guesses.push(Guess {
                    word: G::default(),
                    guess: vec![
                        GuessResult::Empty;
                        self.correct_word.guess(&self.correct_word).guess.len()
                    ],
                });
            }
        }

        guesses
    }

    pub fn end_game(&mut self) {
        self.guesses.clear();
        self.max_tries = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use error::WordleError;
    use guesser::GuessResult;
    use std::error::Error;

    #[test]
    fn test_guess() -> Result<(), Box<dyn Error>> {
        let word_list = vec!["hello".to_string(), "world".to_string()];
        let word = "hello".to_string();
        let mut game = Game::new(5, word, word_list);

        let res = game.guess("hello".to_string())?;
        assert_eq!(
            res.guess,
            vec![
                GuessResult::Correct('h'),
                GuessResult::Correct('e'),
                GuessResult::Correct('l'),
                GuessResult::Correct('l'),
                GuessResult::Correct('o')
            ]
        );
        assert!(game.game_over());

        Ok(())
    }

    #[test]
    fn test_guess_3rd_try() -> Result<(), Box<dyn Error>> {
        let word_list = vec![
            "hello".to_string(),
            "world".to_string(),
            "hella".to_string(),
            "hillo".to_string(),
            "heart".to_string(),
        ];
        let word = "hello".to_string();
        let mut game = Game::new(5, word, word_list);

        let _ = game.guess("world".to_string())?;
        assert!(!game.game_over());

        let res = game.guess("world".to_string());
        assert_eq!(
            res,
            Err(WordleError::WordAlreadyGuessed("world".to_string()))
        );
        assert!(!game.game_over());

        let res = game.guess("hella".to_string())?;
        assert_eq!(
            res.guess,
            vec![
                GuessResult::Correct('h'),
                GuessResult::Correct('e'),
                GuessResult::Correct('l'),
                GuessResult::Correct('l'),
                GuessResult::Incorrect('a')
            ]
        );
        assert!(!game.game_over());

        let res = game.guess("hillo".to_string())?;
        assert_eq!(
            res.guess,
            vec![
                GuessResult::Correct('h'),
                GuessResult::Incorrect('i'),
                GuessResult::Correct('l'),
                GuessResult::Correct('l'),
                GuessResult::Correct('o')
            ]
        );

        assert!(!game.game_over());

        let res = game.guess("hello".to_string())?;
        assert_eq!(
            res.guess,
            vec![
                GuessResult::Correct('h'),
                GuessResult::Correct('e'),
                GuessResult::Correct('l'),
                GuessResult::Correct('l'),
                GuessResult::Correct('o')
            ]
        );
        assert!(game.game_over());

        Ok(())
    }

    #[test]
    fn test_guess_all_wrong() -> Result<(), Box<dyn Error>> {
        let word_list = vec![
            "hello".to_string(),
            "world".to_string(),
            "hella".to_string(),
            "hillo".to_string(),
            "heart".to_string(),
            "beard".to_string(),
        ];

        let word = "hello".to_string();
        let mut game = Game::new(5, word, word_list);
        // attempt to guess the word 5 times
        game.guess("world".to_string())?;
        assert!(!game.game_over());

        game.guess("hella".to_string())?;
        assert!(!game.game_over());

        game.guess("hillo".to_string())?;
        assert!(!game.game_over());

        game.guess("heart".to_string())?;
        assert!(!game.game_over());

        game.guess("beard".to_string())?;

        assert!(game.game_over());
        assert!(game.lost());
        assert!(!game.won());

        Ok(())
    }
}
