use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
    shot::Shot,
    {NUM_COLS, NUM_ROWS},
};
use std::time::Duration;

// Public structure named "Player", 

// pub = public; it can be accessed from other modules outside the one it is defined in
// Omit the "pub" and the structure would be private to this very module.

// struct - keyword defining a structure - a custom data type. It can hold multiple named fields of diff. types.
pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}  

// implementing (???)
impl Player {
    //make a player; return self (this????????) - the player
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,   // roughly in the middle
            y: NUM_ROWS - 1,   // y starts at 0, at the top of the screen. As y inc, we go down on the screen
            shots: Vec::new(),
        }
    }

    // move left
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {  // bool indic. if we successf. shot
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y -1));  //above us
            true
        } else {
            false
        }
    } 

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {  // we go through every shot in our shots; iterate mutably
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());  // cleanup
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    hit_something = true;
                    shot.explode();
                }
            }
        }
        hit_something
    }


}

// draw our player into the frame
impl Drawable for Player {
    // implement draw 
    fn draw(&self, frame: &mut Frame) {
        // we have access to the mutable frame, so we use it
        // we set it to the char that repres. player
        frame[self.x][self.y] = "A";

        // we draw shots here
        for shot in self.shots.iter()  {  // we iterate immutably, bcs we do not need to change values
            shot.draw(frame);
        }

    }
}
