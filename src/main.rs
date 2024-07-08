use std::{
    error::Error,
    sync::mpsc::{self, Receiver},
    time::{Duration, Instant},
    {io, thread},
};
use rusty_audio::Audio;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("win", "win.wav");
    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout(); // initialize
    terminal::enable_raw_mode()?;   // enable key input
    stdout.execute(EnterAlternateScreen)?; // alternate screen. Using extention execute - immediately execute; 
    stdout.execute(Hide)?; // hide cursor

    // Game Loop
    'gameloop: loop {  // thats the name of the game... loop, so we can exit it from anywhere in the loop by name 
        // Input handling / pull for input event. Poll - takes duration. Default duration - zero.
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {}   // if any other key is pressed - ignore
                }
            }
        }
    }   


    // Cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(()) // DO NOT add semicolon here!!!
}
