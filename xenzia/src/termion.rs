extern crate termion;

use termion::input::{ Events, TermRead, Keys};
use termion::event::{ Event, Key };
use termion::{ color, style };
use std::io::{ Write, stdout, stdin };

fn main() {
    /* style and colours.
    println!("{}RED", color::Fg(color::Red));
    println!("{}BLUE", color::Fg(color::Blue));
    println!("{}BLUE'n'BOLD{}", style::Bold, style::Reset);
    println!("{}italic", style::Italic);

    // moving the cursor.
    print!("{}{}Stuff", termion::clear::All, termion::cursor::Goto(1, 1));
    */

    /* reading input.
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let stdin = stdin();
    let mut stdin = stdin.lock();

    stdout.write_all(b"password: ").expect("unable to process password");
    stdout.flush().expect("stdout failure");

    let pass = stdin.read_passwd(&mut stdout);

    if let Ok(Some(pass)) = pass {
        stdout.write_all(pass.as_bytes()).expect("unable to write");
        stdout.write_all(b"\n").expect("unable to write");
    } else {
        stdout.write_all(b"Error\n").expect("shit failed");
    }
    */

    // events,,im specifically interested in key events
    let stdin = stdin();
    let mut stdout = 
}
