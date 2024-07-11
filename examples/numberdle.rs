use rand::seq::IteratorRandom;
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
use rowdle::{guesser::Guess, Game, GuessResult};
use std::io::{self};
use std::io::{BufRead, Stdout};

extern crate rowdle;

fn gen_words(n: usize) -> (String, Vec<String>) {
    // generate all number sequences of length n
    let start = 10u32.pow(n as u32 - 1);
    let end = 10u32.pow(n as u32) - 1;

    let words = (start..=end)
        .map(|i| i.to_string())
        .collect::<Vec<String>>();

    let random = (start..=end)
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();

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

    let mut terminal = setup_terminal()?;

    let (word, word_list) = gen_words(n);

    let mut game = rowdle::Game::new(5, word, word_list);
    let mut input_buffer = String::new();

    loop {
        if !game.game_over() {
            let game_board = game.board(
                Some(0),
                Some(Guess {
                    word: input_buffer.clone(),
                    guess: input_buffer
                        .chars()
                        .map(|c| rowdle::GuessResult::Incorrect(c))
                        .chain(
                            std::iter::repeat(rowdle::GuessResult::Empty)
                                .take(game.correct_word().len() - input_buffer.len()),
                        )
                        .collect(),
                }),
            );
            terminal.draw(|f| {
                ui(f, game_board);
            })?;
            event_handler(&mut game, &mut input_buffer)?;
        } else {
            terminal.draw(|f| {
                play_again_screen(f, f.size(), game.won(), game.correct_word().clone());
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char(c) => {
                            if c == 'y' {
                                let (word, word_list) = gen_words(n);
                                game = rowdle::Game::new(5, word, word_list);
                                input_buffer.clear();
                            } else if c == 'n' {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    shutdown_terminal(terminal)?;
    Ok(())
}

fn event_handler(game: &mut Game<char, String>, buffer: &mut String) -> io::Result<()> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    if c.is_numeric() && buffer.len() < game.correct_word().len() {
                        buffer.push(c);
                    }
                }
                KeyCode::Enter => match game.guess(buffer.clone()) {
                    Ok(_) => {
                        buffer.clear();
                    }
                    Err(e) => {
                        buffer.clear();
                        eprintln!("{}", e);
                    }
                },
                KeyCode::Backspace => {
                    buffer.pop();
                }
                KeyCode::Esc => game.end_game(),
                _ => {}
            }
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, game: Vec<Guess<String, char>>) {
    let main_block = Block::bordered()
        .title("Wordle TUI")
        .title_alignment(Alignment::Center);

    let area = frame.size();
    board(frame, main_block.inner(area), game);
    frame.render_widget(main_block, area);
}

fn board(frame: &mut Frame, rect: Rect, rows: Vec<Guess<String, char>>) {
    let mut constraints = vec![Constraint::Fill(1)];
    rows.iter()
        .for_each(|_| constraints.push(Constraint::Length(3)));
    constraints.push(Constraint::Fill(1));
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(rect);

    for (r, layout) in rows.iter().zip(layout.iter().skip(1)) {
        let row_block = Block::default().borders(Borders::NONE);
        row(frame, row_block.inner(*layout), r.clone());
        frame.render_widget(row_block, *layout);
    }
}

fn row(frame: &mut Frame, rect: Rect, row: Guess<String, char>) {
    let mut constraints = vec![Constraint::Fill(1)];
    row.guess
        .iter()
        .for_each(|_| constraints.push(Constraint::Length(5)));
    constraints.push(Constraint::Fill(1));

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(rect);

    for (c, layout) in row.guess.iter().zip(layout.iter().skip(1)) {
        cell(frame, c, layout);
    }
}

fn cell(frame: &mut Frame, cell: &GuessResult<char>, layout: &Rect) {
    let cell_block = Block::default().borders(Borders::ALL);

    match cell {
        GuessResult::Correct(c) => {
            frame.render_widget(
                Paragraph::new(c.to_string())
                    .alignment(Alignment::Center)
                    .block(cell_block.fg(Color::Indexed(40))),
                *layout,
            );
        }
        GuessResult::Incorrect(c) => {
            frame.render_widget(
                Paragraph::new(c.to_string())
                    .alignment(Alignment::Center)
                    .block(cell_block),
                *layout,
            );
        }
        GuessResult::Misplaced(c) => {
            frame.render_widget(
                Paragraph::new(c.to_string())
                    .alignment(Alignment::Center)
                    .block(cell_block.fg(Color::Indexed(220))),
                *layout,
            );
        }
        GuessResult::Empty => {
            frame.render_widget(
                Paragraph::new("")
                    .alignment(Alignment::Center)
                    .block(cell_block),
                *layout,
            );
        }
        _ => {}
    }
}
fn play_again_screen(frame: &mut Frame, rect: Rect, won: bool, word: String) {
    let text = if won { "You won!" } else { "You lost!" };
    let text = format!("{} The word was: {}", text, word);
    let text = format!("{} \n Play again? [Y/N]", text);
    let text = Span::styled(text, Style::default().fg(Color::White).bg(Color::Black));
    let text = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
    frame.render_widget(text, rect);
}
