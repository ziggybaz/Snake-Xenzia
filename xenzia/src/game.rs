extern crate termion;

use termion::{ raw::IntoRawMode, event::{ Key }, clear, cursor, input::TermRead };
use std::{io::{ stdout, stdin, Read, Write, BufWriter}, collections::VecDeque, time::{ Instant, Duration}, thread::sleep };
use rand::Rng;

const GRID_WIDTH:u16 = 40;
const GRID_HEIGHT:u16 = 20;

fn game_grid(stdout: &mut std::io::Stdout) {
    write!(stdout, "{}", cursor::Goto(1, 1)).expect("Unable to position cursor to draw top border");
    print!("+");
    for _ in 0..GRID_WIDTH {
        print!("=");
    }
    print!("+");

    for y in 2..=GRID_HEIGHT + 1 {
        write!(stdout, "{}", cursor::Goto(1, y)).expect("Unable to position cursor to draw side border");
        print!("#");
        write!(stdout, "{}", cursor::Goto(GRID_WIDTH + 2, y)).expect("fuck, i'll add proper error-handling, tired with thess expect blocks, will just use unwraps as placeholders fo now");
        println!("#");
    }

    write!(stdout, "{}", cursor::Goto(1, GRID_HEIGHT + 2)).unwrap();
    print!("+");
    for _ in 0..GRID_WIDTH {
        print!("=");
    }
    println!("+");

    stdout.flush().unwrap();
}

fn draw_food(stdout: &mut std::io::Stdout, x: u16, y: u16) {
    write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
    print!("*");
}

fn food_position(x: &mut u16, y: &mut u16, snake: &VecDeque<(u16, u16)>) {
    let mut rng = rand::thread_rng();
    loop {
        *x = rng.gen_range(2..=GRID_WIDTH);
        *y = rng.gen_range(2..=GRID_HEIGHT);

        if !snake.contains(&(*x, *y)) {
            break;
        }
    }
}


fn draw_snake(stdout: &mut std::io::Stdout, snake: &VecDeque<(u16, u16)>) {
    for &(x, y) in snake {
        write!(stdout, "{}", cursor::Goto(x, y)).unwrap();
        print!("O=");
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

fn detect_collision(snake: &VecDeque<(u16, u16)>) -> bool {
    let head = *snake.front().unwrap();

    if head.0 < 2 || head.0 > GRID_WIDTH + 1 || head.1 < 2 || head.1 > GRID_HEIGHT + 1 {
        return true;
    }

    if snake.iter().skip(1).any(|&body| body == head) {
        return true;
    }

    false
}

fn consume_food(snake: &mut VecDeque<(u16, u16)>, food_x: &mut u16, food_y: &mut u16, stdout: &mut std::io::Stdout) {
    let head = *snake.front().unwrap();

    if head == (*food_x, *food_y) {
        let tail = *snake.back().unwrap();
        snake.push_back(tail);

        food_position(food_x, food_y, snake);
        draw_food(stdout, *food_x, *food_y);
    }
}

fn game_loop() {
    let stdin = stdin();
    let mut stdin_keys = stdin.keys();
    let stdout = stdout();
    let mut raw_stdout = stdout.into_raw_mode().unwrap();

    let mut food_x = 1;
    let mut food_y = 1;

    let mut snake = VecDeque::new();
    let snake_center_x = GRID_WIDTH / 2+ 1;
    let snake_center_y = GRID_HEIGHT / 2 + 1;
    snake.push_back((snake_center_x, snake_center_y));

    food_position(&mut food_x, &mut food_y, &snake);
    game_grid(&mut raw_stdout);
    draw_snake(&mut raw_stdout, &snake);
    draw_food(&mut raw_stdout, food_x, food_y);

    raw_stdout.flush().expect("unable to display output");

    loop {
        if let Some(Ok(key)) = stdin_keys.next() {
            match key {
                Key::Up => snake_movement(&mut snake, Direction::Up),
                Key::Down => snake_movement(&mut snake, Direction::Down),
                Key::Left => snake_movement(&mut snake, Direction::Left),
                Key::Right => snake_movement(&mut snake, Direction::Right),
                Key::Ctrl('c') => break,
                _ => {}
            }
        }

        consume_food(&mut snake, &mut food_x, &mut food_y, &mut raw_stdout);

        if detect_collision(&snake) {
            println!("GAME OVER");
            break;
        }

        write!(raw_stdout, "{}", clear::All).unwrap();
        game_grid(&mut raw_stdout);
        draw_snake(&mut raw_stdout, &snake);
        draw_food(&mut raw_stdout, food_x, food_y);

        raw_stdout.flush().unwrap();

        sleep(Duration::from_millis(100));
    }
}

pub fn initialize_game() {
    let stdout = stdout();
    let mut raw_stdout = stdout.into_raw_mode().unwrap();

    write!(raw_stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    write!(raw_stdout, "{}", cursor::Goto(1, 1)).unwrap();

    let mut food_x = 1;
    let mut food_y = 1;

    let snake_center_x = GRID_WIDTH / 2 + 1;
    let snake_center_y = GRID_HEIGHT / 2 + 1;
    let mut snake: VecDeque<(u16, u16)> = VecDeque::new();
    snake.push_back((snake_center_x, snake_center_y));

    food_position(&mut food_x, &mut food_y, &snake);
    game_grid(&mut raw_stdout);
    draw_snake(&mut raw_stdout, &snake);
    draw_food(&mut raw_stdout, food_x, food_y);

    raw_stdout.flush().expect("unable to display output");

    game_loop();
}

/// pause_game
fn pause_game() {
}

/// end/quit game
/// user presses esc to quit game abruptly, give option to restart or quit game
/// diplay final score upon collision
fn end_game() {
    unimplemented!();
}


#[cfg(test)]
mod tests {
    use super::*;
}
