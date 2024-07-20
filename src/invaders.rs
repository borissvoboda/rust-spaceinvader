use rusty_time::timer::Timer;
use crate::{NUM_ROWS, NUM_COLS};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invader {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

// logic

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                if ( x > 1)
                    && (x < NUM_COLS - 2)
                    && (y > 0)
                    && (y < 9)  // stop in the middle 
                    && (x % 2 == 0)
                    && (y % 2 == 0) {
                        army.push(Invader { x, y});
                    }

            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(2000),
            direction: 1,
        }
    }

    // update
    pub fn update(&mut self, delta: Duration) -> bool {  // wheather 
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwarrds = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);    
                if min_x == 0 {
                    self.direction = 1; // change direction
                    downwards = true;
                }
            } else {  // for the right side
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }
            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250) // everytime we move downwoards we inc their speed, if it goes below 250, return to 250
                self.move_timer = Timer::from_millis(new_duration as u64);
                for invader in self.army.iter_mut() { // loop through every invader
                    invader.y += 1;
                }
            } else {  // side
                for invader in self.army.iter_mut() { 
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }
            return true;
        }
        false
    }

}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {  // draw each individ invader. Immutably, we dont need to change them
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()    // we make invaders wave their arms. Half of the time we show one char, the other half - the other one.
            / self.move_timer.duration().as_secs_f32())
            > 0.5
            {
                'x'
            } else {
                '+'
            }
        }
    }
}