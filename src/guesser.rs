#[derive(Debug, Clone, PartialEq)]
pub enum GuessResult {
    Correct,
    Incorrect,
    Misplaced,
    Custom(String),
}

pub trait Guessable: PartialEq + Clone {
    fn guess(&self, other: &Self) -> Vec<GuessResult>;
}

impl Guessable for &str {
    fn guess(&self, other: &Self) -> Vec<GuessResult> {
        if self == other {
            return vec![GuessResult::Correct; self.len()];
        }

        // initialize result vector with incorrect guesses
        let mut result = vec![GuessResult::Incorrect; self.len()];
        let mut remaining_word = String::new();

        // check for correct guesses
        for (i, c) in self.chars().enumerate() {
            if other.chars().nth(i) == Some(c) {
                result[i] = GuessResult::Correct;
            } else {
                result[i] = GuessResult::Incorrect;
                remaining_word.push(other.chars().nth(i).unwrap());
            }
        }

        for (i, c) in result.iter_mut().enumerate() {
            if let GuessResult::Incorrect = c {
                let char = self.chars().nth(i).unwrap();
                if remaining_word.contains(char) {
                    *c = GuessResult::Misplaced;
                    remaining_word.remove(remaining_word.find(char).unwrap());
                }
            }
        }

        return result;
    }
}

impl Guessable for String {
    fn guess(&self, other: &Self) -> Vec<GuessResult> {
        if self == other {
            return vec![GuessResult::Correct; self.len()];
        }

        // initialize result vector with incorrect guesses
        let mut result = vec![GuessResult::Incorrect; self.len()];
        let mut remaining_word = String::new();

        // check for correct guesses
        for (i, c) in self.chars().enumerate() {
            if other.chars().nth(i) == Some(c) {
                result[i] = GuessResult::Correct;
            } else {
                result[i] = GuessResult::Incorrect;
                remaining_word.push(other.chars().nth(i).unwrap());
            }
        }

        for (i, c) in result.iter_mut().enumerate() {
            if let GuessResult::Incorrect = c {
                let char = self.chars().nth(i).unwrap();
                if remaining_word.contains(char) {
                    *c = GuessResult::Misplaced;
                    remaining_word.remove(remaining_word.find(char).unwrap());
                }
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess() {
        let guess = "world";
        let correct = "hello";
        let res = guess.guess(&correct);
        assert_eq!(
            res,
            vec![
                GuessResult::Incorrect,
                GuessResult::Misplaced,
                GuessResult::Incorrect,
                GuessResult::Correct,
                GuessResult::Incorrect
            ]
        );

        let guess = "flour";
        let correct = "level";
        let res = guess.guess(&correct);
        assert_eq!(
            res,
            vec![
                GuessResult::Incorrect,
                GuessResult::Misplaced,
                GuessResult::Incorrect,
                GuessResult::Incorrect,
                GuessResult::Incorrect
            ]
        );

        let guess = "level";
        let correct = "flour";

        let res = guess.guess(&correct);
        assert_eq!(
            res,
            vec![
                GuessResult::Misplaced,
                GuessResult::Incorrect,
                GuessResult::Incorrect,
                GuessResult::Incorrect,
                GuessResult::Incorrect
            ]
        );
    }
}
