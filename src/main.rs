use std::{
    error::Error, io::{self, Stdout}, sync::mpsc::{self, channel, Receiver}, thread, time::{Duration, Instant}
};
use invaders::{
    frame::{self, new_frame, Drawable, Frame},
    invaders::Invaders,
    // level::Level,
    // menu::Menu,
    player::Player,
    render,
    // score::Score,
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


    // ------------------------------------------
    // Render loop in a separate thread
    // Multithreading
    // marginal speed up
    // channel to communicate with the thread
    // render tx = render transciever 
    // render rx = render receiver
    // mpsc channels are built in the standard library
    let (render_tx, render_rx) = mpsc::channel();
    // catching thread handler in this var; standard thread 
    // it teakes closure
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();

        // actually rendering the entire screen once 
        // giving it a mutable ref. to stdout, immutable ref to last_frame.
        // since we dont have a current frame, so we give it last frame again
        // last "true" = force rendering everything
        render::render(&mut stdout, &last_frame, &last_frame, true);
        // screen has beet set up once.
        // now we can do incremental updates
        loop {
            // we match on the result - if it is a frame, or an error.
            // this will return a current frame, but to do smtg with it, we make
            // a var from it - with let 
           let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;

        }
    });



    // Game Loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    let mut invaders = Invaders::new();
    'gameloop: loop {  // thats the name of the game... loop, so we can exit it from anywhere in the loop by name 
        
        // Per-frame initialization
        let delta = instant.elapsed();  // since the begin. of its lifetime.
        instant = Instant::now();                   // next time we are in the loop, we have measured the time we are in the loop
        let mut curr_frame = new_frame();


        // INPUT
        // Input handling / pull for input event. Poll - takes duration. Default duration - zero.
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    },
                    _ => {}   // if any other key is pressed - ignore
                }
            }
        }

        // --------------------------------------------------------------
        // UPDATES - updatig timers
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move");
        }


        // --------------------------------------------------------------
        // Draw & render section
        
        // Draw player into the frame
        // player.draw(&mut curr_frame);     // no longer needed
        // invaders.draw(&mut curr_frame);   // no longer needed
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }

        // 1. render transciever side; send current frame
        // we dont need it, so we move it to a diff thread
        // this returns a result; but we expect it to fail a first few times
        // this game loop keeps going before that child thread is set up and starts receiving
        // so we ignore the errror silently ( " let _ ")...
        let _ = render_tx.send(curr_frame);  
        // artificial sleep - single milisecond
        thread::sleep(Duration::from_millis(1));

    }   


    // Cleanup
    drop(render_tx);  // we need to join the thread. Newer versions of rust do not require this
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(()) // DO NOT add semicolon here!!!
}
