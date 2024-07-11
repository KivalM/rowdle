#[derive(Debug, Clone, PartialEq)]
pub enum GuessResult<T: PartialEq> {
    Correct(T),
    Incorrect(T),
    Misplaced(T),
    Empty,
    Custom(T),
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Guess<T: PartialEq + Clone, G: PartialEq + Clone> {
    pub word: T,
    pub guess: Vec<GuessResult<G>>,
}

pub trait Guessable<T: PartialEq + Clone>: PartialEq + Clone {
    fn guess(&self, other: &Self) -> Guess<Self, T>;
}

impl Guessable<char> for String {
    fn guess(&self, other: &Self) -> Guess<String, char> {
        if self == other {
            return Guess {
                word: self.clone(),
                guess: other.chars().map(|c| GuessResult::Correct(c)).collect(),
            };
        }

        // initialize result vector with incorrect guesses
        let mut result = vec![GuessResult::Incorrect(' '); self.len()];
        let mut remaining_word = String::new();

        // check for correct guesses
        for (i, c) in self.chars().enumerate() {
            if other.chars().nth(i) == Some(c) {
                result[i] = GuessResult::Correct(c);
            } else {
                result[i] = GuessResult::Incorrect(c);
                remaining_word.push(other.chars().nth(i).unwrap());
            }
        }

        for (i, c) in result.iter_mut().enumerate() {
            if let GuessResult::Incorrect(_) = c {
                let char = self.chars().nth(i).unwrap();
                if remaining_word.contains(char) {
                    *c = GuessResult::Misplaced(char);
                    remaining_word.remove(remaining_word.find(char).unwrap());
                }
            }
        }

        return Guess {
            guess: result,
            word: self.clone(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess() {
        let guess = "world".to_string();
        let correct = "hello".to_string();
        let res = guess.guess(&correct);
        assert_eq!(
            res.guess,
            vec![
                GuessResult::Incorrect('w'),
                GuessResult::Misplaced('o'),
                GuessResult::Incorrect('r'),
                GuessResult::Correct('l'),
                GuessResult::Incorrect('d')
            ]
        );

        let guess = "flour".to_string();
        let correct = "level".to_string();
        let res = guess.guess(&correct);
        assert_eq!(
            res.guess,
            vec![
                GuessResult::Incorrect('f'),
                GuessResult::Misplaced('l'),
                GuessResult::Incorrect('o'),
                GuessResult::Incorrect('u'),
                GuessResult::Incorrect('r')
            ]
        );

        let guess = "level".to_string();
        let correct = "flour".to_string();

        let res = guess.guess(&correct);
        assert_eq!(
            res.guess,
            vec![
                GuessResult::Misplaced('l'),
                GuessResult::Incorrect('e'),
                GuessResult::Incorrect('v'),
                GuessResult::Incorrect('e'),
                GuessResult::Incorrect('l')
            ]
        );
    }
}
