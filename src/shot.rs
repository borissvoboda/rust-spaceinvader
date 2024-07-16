use crate::frame::{Drawable, Frame};
use rusty_time::Timer;
use std::time::Duration;

pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::new(Duration::from_millis(50)),

        }
    }

    // updating the timer
    pub fn update(&mut self, delta: Duration) {
            self.timer.tick(delta);  // makes timer start counting down by delta amount
            if self.timer.finished() && !self.exploding {    // if timer is ready and we are not exploding, we can move
                if self.y > 0 {  // if we havent reached the top of the screen, we can move up
                    self.y -= 1;
                }  // we moved if thats the case
                self.timer.reset();
            }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        // setting timer so we can see the explosion
        self.timer = Timer::new(Duration::from_millis(250));
    }

    // function to tell when we are dead
    // dead or not -> bool
    pub fn dead(&self) -> bool {
        // we exploded and time ran out OR if we reached the top of the screen
        (self.exploding && self.timer.finished()) || (self.y == 0)
    }
}

// so we can see the shot
impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding { "*" } else { "|"};

        }
}

