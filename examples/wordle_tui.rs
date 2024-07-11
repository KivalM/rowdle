use random_word::Lang;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::*,
    widgets::*,
};
use std::io::{self};
use std::io::{BufRead, Stdout};

extern crate rowdle;

fn gen_words(n: usize) -> (String, Vec<String>) {
    let words = random_word::all_len(n, Lang::En).unwrap();
    // to vec of strings
    let words = words.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    let random = random_word::gen_len(n, Lang::En).unwrap().to_string();

    (random, words)
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    Ok(terminal)
}

fn shutdown_terminal(
    mut terminal: Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Wordle!");
    println!("How long should the word be?");
    let n: usize = std::io::stdin().lock().lines().next().unwrap()?.parse()?;

    let terminal = setup_terminal()?;

    let (word, word_list) = gen_words(n);

    let mut game = rowdle::Game::new(5, word, word_list);

    while !game.game_over() {
        println!("Enter your guess:");
        let guess = std::io::stdin().lock().lines().next().unwrap()?;

        match game.guess(guess.clone()) {
            Ok(res) => {
                for r in res {
                    print!("{:?} ", r);
                }
                println!();
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, game: &Vec<Vec<Option<GuessCell>>>) {
    let main_block = Block::bordered()
        .title("Wordle TUI")
        .title_alignment(Alignment::Center);

    // frame.render_widget(main_block, frame.size());
    let area = frame.size();
    board(frame, main_block.inner(area), game);

    frame.render_widget(main_block, area);
}
