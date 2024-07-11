use guesser::{GuessResult, Guessable};

pub mod error;
pub mod guesser;

pub struct Game<T: Guessable> {
    max_tries: u8,
    correct_word: T,
    word_list: Vec<T>,
    guesses: Vec<T>,
}

impl<T: Guessable> Game<T> {
    pub fn new(max_tries: u8, correct_word: T, word_list: Vec<T>) -> Self {
        Self {
            max_tries,
            correct_word,
            word_list,
            guesses: vec![],
        }
    }

    pub fn guess(&mut self, word: T) -> Result<Vec<GuessResult>, error::WordleError<T>> {
        if self.max_tries == 0 {
            return Err(error::WordleError::MaxTriesExceeded);
        }

        if !self.word_list.contains(&word) {
            return Err(error::WordleError::InvalidWord(word));
        }

        if self.guesses.contains(&word) {
            return Err(error::WordleError::WordAlreadyGuessed(word));
        }
        let res = word.guess(&self.correct_word);
        self.guesses.push(word.clone());
        Ok(res)
    }

    pub fn won(&self) -> bool {
        self.guesses.contains(&self.correct_word)
    }

    pub fn lost(&self) -> bool {
        self.guesses.len() == self.max_tries.into() && !self.guesses.contains(&self.correct_word)
    }

    pub fn game_over(&self) -> bool {
        self.won() || self.lost()
    }

    pub fn correct_word(&self) -> &T {
        &self.correct_word
    }

    pub fn board(&self) -> Vec<(T, Vec<GuessResult>)> {
        self.guesses
            .iter()
            .map(|g| (g.clone(), g.guess(&self.correct_word)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use error::WordleError;
    use std::error::Error;

    #[test]
    fn test_guess() -> Result<(), Box<dyn Error>> {
        let word_list = vec!["hello", "world"];
        let word = "hello";
        let mut game = Game::new(5, word, word_list);

        let res = game.guess("hello")?;
        assert_eq!(res, vec![GuessResult::Correct; 5]);
        assert!(game.game_over());

        Ok(())
    }

    #[test]
    fn test_guess_3rd_try() -> Result<(), Box<dyn Error>> {
        let word_list = vec!["hello", "world", "hella", "hillo", "heart"];
        let word = "hello";
        let mut game = Game::new(5, word, word_list);

        let _ = game.guess("world")?;
        assert!(!game.game_over());

        let res = game.guess("world");
        assert_eq!(res, Err(WordleError::WordAlreadyGuessed("world")));
        assert!(!game.game_over());

        let res = game.guess("hella")?;
        assert_eq!(
            res,
            vec![
                GuessResult::Correct,
                GuessResult::Correct,
                GuessResult::Correct,
                GuessResult::Correct,
                GuessResult::Incorrect
            ]
        );
        assert!(!game.game_over());

        let res = game.guess("hillo")?;
        assert_eq!(
            res,
            vec![
                GuessResult::Correct,
                GuessResult::Incorrect,
                GuessResult::Correct,
                GuessResult::Correct,
                GuessResult::Correct
            ]
        );

        assert!(!game.game_over());

        let res = game.guess("hello")?;
        assert_eq!(res, vec![GuessResult::Correct; 5]);
        assert!(game.game_over());

        Ok(())
    }

    #[test]
    fn test_guess_all_wrong() -> Result<(), Box<dyn Error>> {
        let word_list = vec!["hello", "world", "hella", "hillo", "heart", "beard"];

        let word = "hello";
        let mut game = Game::new(5, word, word_list);
        // attempt to guess the word 5 times
        game.guess("world")?;
        assert!(!game.game_over());

        game.guess("hella")?;
        assert!(!game.game_over());

        game.guess("hillo")?;
        assert!(!game.game_over());

        game.guess("heart")?;
        assert!(!game.game_over());

        game.guess("beard")?;

        assert!(game.game_over());
        assert!(game.lost());
        assert!(!game.won());

        Ok(())
    }
}
