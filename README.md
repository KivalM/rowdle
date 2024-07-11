# Rowdle
A simple, fast and lightweight backend for wordle-based games.
Think of it as a wordle clone able to do more than just words, loldle, numberdle, etc. (See the examples directory for some of the possibilities)

## Installation
```bash
cargo add rowdle
```

## Usage
```rust
// initialize a game with 5 letters and the word "hello" as the target
// the word list is a list of possible words that the target can be
let mut game = rowdle::Game::new(5, "guess", vec![
    "hello",
    "world",
    "rust",
    "rowdl",
    "wordl",
]);

let guess = "world";

let result = game.guess(guess);

println!("{:?}", result.board(None,None));
```

