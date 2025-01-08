extern crate termion;

use termion::{
    raw::IntoRawMode,
    event::{ Key },
    clear,
    cursor,
    input::TermRead,
    color};

use std::{
    io::{ stdout, stdin, BufRead, BufReader, Write },
    fs::OpenOptions,
    collections::VecDeque,
    time::{ Instant, Duration},
    thread::sleep };

use rand::Rng;


fn game_grid(stdout: &mut std::io::Stdout, width: u16, height: u16) {
    write!(stdout, "{}", cursor::Goto(1, 1)).expect("Unable to position cursor to draw top border");
    print!("##");
    for _ in 0..width {
        print!("##");
    }
    print!("##");

    for y in 2..=height + 1 {
        write!(stdout, "{}##", cursor::Goto(1, y)).expect("Unable to position cursor to draw side border");
        write!(stdout, "{}##", cursor::Goto(width + 2, y)).expect("fuck, i'll add proper error-handling, tired with thess expect blocks, will just use unwraps as placeholders fo now");
    }

    write!(stdout, "{}", cursor::Goto(1, height + 2)).unwrap();
    print!("##");
    for _ in 0..width {
        print!("##");
    }
    println!("##");

    stdout.flush().unwrap();
}

fn draw_food(stdout: &mut std::io::Stdout, x: u16, y: u16) {
    write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
    print!("*");
}

fn food_position(x: &mut u16, y: &mut u16, snake: &VecDeque<(u16, u16)>, width: u16, height: u16) {
    let mut rng = rand::thread_rng();
    loop {
        *x = rng.gen_range(2..=width);
        *y = rng.gen_range(2..=height);

        if !snake.contains(&(*x, *y)) {
            break;
        }
    }
}


fn draw_snake(stdout: &mut std::io::Stdout, snake: &VecDeque<(u16, u16)>) {
    if let Some(&(head_x, head_y)) = snake.front() {
        write!(stdout, "{}O", cursor::Goto(head_x, head_y)).unwrap();
    }

    for &(x, y) in snake.iter().skip(1) {
        write!(stdout, "{}=", cursor::Goto(x, y)).unwrap();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
fn snake_movement(snake: &mut VecDeque<(u16, u16)>, direction: Direction) {
    let head = *snake.front().unwrap();

    let new_head = match direction {
        Direction::Up => (head.0, head.1 - 1),
        Direction::Down => (head.0, head.1 + 1),
        Direction::Left => (head.0 - 1, head.1),
        Direction::Right => (head.0 + 1, head.1),
    };

    snake.push_front(new_head);
    snake.pop_back();
}

fn detect_collision(snake: &VecDeque<(u16, u16)>, width: u16, height: u16) -> bool {
    let head = *snake.front().unwrap();

    if head.0 < 2 || head.0 > width + 1 || head.1 < 2 || head.1 > height + 1 {
        return true;
    }

    if snake.iter().skip(1).any(|&body| body == head) {
        return true;
    }

    false
}

fn consume_food(
    snake: &mut VecDeque<(u16, u16)>,
    food_x: &mut u16,
    food_y: &mut u16,
    stdout: &mut std::io::Stdout,
    width: u16,
    height: u16) {
    let head = *snake.front().unwrap();

    if head == (*food_x, *food_y) {
        let tail = *snake.back().unwrap();
        snake.push_back(tail);

        food_position(food_x, food_y, snake, width, height);
        draw_food(stdout, *food_x, *food_y);
    }
}

fn display_text(stdout: &mut std::io::Stdout, lines: &[&str], color_code: color::Rgb) {
    let terminal_size = termion::terminal_size().unwrap();
    let terminal_width = terminal_size.0;
    let terminal_height = terminal_size.1;

    let start_y = (terminal_height / 2) - (lines.len() as u16 / 2);
    for (i, line) in lines.iter().enumerate() {
        let x = (terminal_width / 2) - (line.len() as u16 / 2);
        write!(stdout, "{}{}{}{}", cursor::Goto(x, start_y + i as u16), color::Fg(color_code), line, color::Fg(color::Reset)).unwrap();
    }
    stdout.flush().unwrap();
}

fn welcome_screen() {
    let stdout = stdout();
    let mut raw_stdout = stdout.into_raw_mode().unwrap();

    write!(raw_stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    let welcome_text = [
        "Welcome To SNAKE XENZIA", // i know the name is trademarked but I'm still going to use it,
        "",
        "INSTRUCTIONS:",
        " - Use Arrow Keys to move the snake.",
        " - Collect the food to grow your snake.",
        " - Avoid colliding with the borders and crushing into yourself",
        "",
        "Press 'ENTER' to start..."
    ];

    display_text(&mut raw_stdout, &welcome_text, color::Rgb(0, 0, 255));

    let stdin = stdin();
    for key in stdin.keys() {
        if let Ok(Key::Char('\n')) = key {
            break;
        }
    }

    write!(raw_stdout, "{}", clear::All).unwrap();
    raw_stdout.flush().unwrap();
}

fn pause_screen() -> Option<String> {
    let stdout = stdout();
    let mut raw_stdout = stdout.into_raw_mode().unwrap();

    write!(raw_stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    let pause_text = [
        "Game Paused",
        "",
        "Press [C] to Continue, [R] to Restart, or [E] to Exit",
    ];

    display_text(&mut raw_stdout, &pause_text, color::Rgb(255, 255, 0));

    let stdin = stdin();
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('C') | Key::Char('c') => return Some("continue".to_string()),
            Key::Char('R') | Key::Char('r') => return Some("restart".to_string()),
            Key::Char('E') | Key::Char('e') => return Some("exit".to_string()),
            _ => {}
        }
    }
    None
}

fn game_over_screen(score: u32) -> bool {
    let stdout = stdout();
    let mut raw_stdout = stdout.into_raw_mode().unwrap();

    write!(raw_stdout, "{}{}", clear::All, cursor::Hide).unwrap();

    let game_over_text = [
        "GAME OVER",
        "",
        "Press [R] to Restart or [E] to Exit"
    ];

    score_tracker(score);
    display_text(&mut raw_stdout, &game_over_text, color::Rgb(255, 0, 0));

    let stdin = stdin();
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char('R') | Key::Char('r') => return true,
            Key::Char('E') | Key::Char('e') => return false,
            _ => {}
        }
    }
    false
}

fn score_tracker(score: u32) {
    const SCORE_FILE: &str = "scores.txt";
    const MAX_SCORES: usize = 5;

    let mut scores = Vec::new();
    if let Ok(file) = OpenOptions::new().read(true).open(SCORE_FILE) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Ok(num) = line.trim().parse::<u32>() {
                    scores.push(num)
                }
            }
        }
    }

    scores.push(score);
    scores.sort_unstable_by(|a, b| b.cmp(a));
    scores.truncate(MAX_SCORES);

    if let Ok(mut file) = OpenOptions::new().write(true).create(true).truncate(true).open(SCORE_FILE) {
        for score in &scores {
            writeln!(file, "{}", score).unwrap();
        }
    }
}

fn display_score(stdout: &mut std::io::Stdout, score: u32) {
    let score_message = [
        &format!("Your Score: {}", score),
        "",
        "Press [R] to Restart or [E] to Exit"
    ];

    display_text(stdout, &score_message, color::Rgb(255, 255, 0));
}

fn game_loop() -> (bool, u32) {
    let (width, height) = termion::terminal_size().unwrap();
    let grid_width = width - 2;
    let grid_height = height - 3;


    let stdin = stdin();
    let mut stdin_keys = stdin.keys();
    let stdout = stdout();
    let mut raw_stdout = stdout.into_raw_mode().unwrap();

    let mut food_x = 1;
    let mut food_y = 1;

    let mut snake = VecDeque::new();
    let snake_center_x = grid_width / 2 + 1;
    let snake_center_y = grid_height / 2 + 1;
    snake.push_back((snake_center_x, snake_center_y));

    food_position(&mut food_x, &mut food_y, &snake, grid_width, grid_height);
    game_grid(&mut raw_stdout, grid_width, grid_height);
    draw_snake(&mut raw_stdout, &snake);
    draw_food(&mut raw_stdout, food_x, food_y);

    raw_stdout.flush().expect("unable to display output");

    let mut direction = Direction::Right;
    let mut score = 0;

    loop {
        if let Some(Ok(key)) = stdin_keys.next() {
            match key {
                Key::Up if direction != Direction::Down => direction = Direction::Up,
                Key::Down if direction != Direction::Up => direction = Direction::Down,
                Key::Left if direction != Direction::Right => direction = Direction::Left,
                Key::Right if direction != Direction::Left => direction = Direction::Right,
                Key::Esc => {
                    if let Some(choice) = pause_screen() {
                        match choice.as_str() {
                            "continue" => continue,
                            "restart" => return (true, score),
                            "exit" => return (false, score),
                            _ => {}
                        }
                    }
                }
                Key::Ctrl('c') => return (false, score),
                _ => {}
            }
        }

        snake_movement(&mut snake, direction);

        if detect_collision(&snake, grid_width, grid_height) {
            return (game_over_screen(score), score);
        }
        if *snake.front().unwrap() == (food_x, food_y) {
            score += 1;
        }

        consume_food(&mut snake, &mut food_x, &mut food_y, &mut raw_stdout, grid_width, grid_height);

        write!(raw_stdout, "{}", clear::All).unwrap();
        game_grid(&mut raw_stdout, grid_width, grid_height);
        draw_snake(&mut raw_stdout, &snake);
        draw_food(&mut raw_stdout, food_x, food_y);

        raw_stdout.flush().unwrap();

        sleep(Duration::from_millis(100));
    }
}

pub fn initialize_game() {
    welcome_screen();

    loop {
        let (restart, score) = game_loop();
        if !restart {
            break;
        }

        if !game_over_screen(score) {
            break;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
}
