use std::io::Stdout;
use std::io::Write;
use crate::frame::Frame;
use crossterm::QueueableCommand;
use crossterm::style::{SetBackgroundColor, Color};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::MoveTo;



pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap(); // unwrap - crash???
        stdout.queue(Clear(ClearType::All)).unwrap();  // Why do we need the clear operation??? 
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    // we iterate through our entire frame 
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            // dereference one level - access the value that a reference is pointing to
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}