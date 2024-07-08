// frame module; 
// Frame = vector of vectors of borrowed, static string slices...

use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;   // type alias; vector of vectors!!! borrowed static string slices


// will generate a new frame
pub fn new_frame() -> Frame {  
    let mut cols= Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col= Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" "); // blank frame???
        }
        cols.push(col);
    }
    cols
}

// pub trait 

/*
vector of vectors of borrowed static string slices =
according to Rust docs and AI:

1. Vector 
Vec<T>
- re-sizable array.  https://doc.rust-lang.org/rust-by-example/std/vec.html
- is a growable array type; part of the standard lib.

2. Borrowed - data referenced by the string slices is not owned by the vectors, but rather borrowed from elsewhere.
(ok, so the ownership thing).
- creating a reference - borrowing

3. Static - lifetime... 
https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html

4. String slices... - are of type &str. They are references to a sequence of UTF-8 encoded characters.

*/