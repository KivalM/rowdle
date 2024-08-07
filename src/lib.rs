pub use guesser::{Guess, GuessResult, Guessable};
use std::fmt::Debug;

pub mod error;
pub mod guesser;

/// A game of Wordle
///
/// # Example
///
/// ```
/// use rowdle::guesser::GuessResult;
/// use rowdle::{Game};
/// let word_list = vec!["hello".to_string(), "world".to_string()];
/// let word = "hello".to_string();
/// let mut game = Game::new(5, word, word_list);

/// let res = game.guess("hello".to_string()).unwrap();
/// assert_eq!(
///     res.guess,
///     vec![
///         GuessResult::Correct('h'),
///         GuessResult::Correct('e'),
///         GuessResult::Correct('l'),
///         GuessResult::Correct('l'),
///         GuessResult::Correct('o')
///     ]
/// );
/// assert!(game.game_over());
///
pub struct Game<T: PartialEq + Clone, G: Guessable<T> + Default + Debug> {
    max_tries: u8,
    correct_word: G,
    word_list: Vec<G>,
    guesses: Vec<Guess<G, T>>,
}

impl<T: PartialEq + Clone, G: Guessable<T> + Default + Debug> Game<T, G> {
    /// Create a new game of Wordle
    /// # Arguments
    /// * `max_tries` - The maximum number of tries allowed
    /// * `correct_word` - The correct word to guess
    /// * `word_list` - A list of words that can be guessed
    pub fn new(max_tries: u8, correct_word: G, word_list: Vec<G>) -> Self {
        Self {
            max_tries,
            correct_word,
            word_list,
            guesses: vec![],
        }
    }

    /// Make a guess
    /// # Arguments
    /// * `word` - The word to guess
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

    /// Check if a word has been guessed
    pub fn is_word_guessed(&self, word: &G) -> bool {
        self.guesses.iter().any(|g| g.word == *word)
    }

    /// Check if the game is won
    /// A game is won if the correct word has been guessed
    pub fn won(&self) -> bool {
        self.guesses.iter().any(|g| g.word == self.correct_word)
    }

    /// Check if the game is lost
    /// A game is lost if the maximum number of tries has been exceeded
    pub fn lost(&self) -> bool {
        self.guesses.len() == self.max_tries.into() && !self.won()
    }

    /// Check if the game is over
    /// A game is over if the game is won or lost
    pub fn game_over(&self) -> bool {
        self.won() || self.lost()
    }

    /// Get the correct word
    pub fn correct_word(&self) -> &G {
        &self.correct_word
    }

    /// get a 2d vector of the board
    /// # Arguments
    /// * `pad` - The number of empty guesses to pad the board with
    /// * `buffer` - A buffer guess to add to the board
    /// # Returns
    /// A 2d vector of the board
    pub fn board(&self, pad: Option<u32>, buffer: Option<Guess<G, T>>) -> Vec<Guess<G, T>> {
        let mut guesses = self.guesses.clone();

        if let Some(buffer) = buffer {
            let len = self.correct_word.guess(&self.correct_word).guess.len();
            // pad the buffer guess with empty guesses
            let n = len - buffer.guess.len();
            let mut guess = buffer.clone();
            for _ in 0..n {
                guess.guess.push(GuessResult::Empty);
            }

            guesses.push(guess);
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

    /// End the game
    /// Clear the guesses and set the maximum number of tries to 0
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
