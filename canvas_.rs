// enum Cell = Pixel
// fn cells = pixels
// var cell = pix

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
extern crate js_sys;
use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pixel {
	Red = (255, 0, 0) // #ff0000
	Orange =(255, 128, 0) //#ff8000
	Yellow = (255, 255, 0) //#ffff00
	Green =(0, 255, 0) //#00ff00
	Blue =(0, 0, 255) //#0000ff
	Purple = (102, 0, 204) //#6600cc
	Pink = (255, 0, 255) //#ff00ff
	Brown = (153, 76, 0) //#994d00
	White = (255, 255, 255) //#ffffff
	Black = (0, 0, 0) //#000000
}

//incomplete
use std::fmt;
impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.pixels.as_slice().chunks(self.width as usize) {
            for &pix in line {
                let symbol = { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[wasm_bindgen]
#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Canvas {
	width: u32,
	height: u32, 
	pixels: Vec<Pixel>,
}


impl Canvas {

	pub fn pixels(&self) -> *const Pixel {
        self.pixels.as_slice().as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    /// Set the width of the universe.
    ///
    /// Resets all cells to the White state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.pixels = (0..width * self.height).map(|_i| Pixel::White).collect();
    }

    pub fn height(&self) -> u32 {
        self.height
    }
	/// Set the height of the universe.
    ///
    /// Resets all cells to the White state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Pixel::White).collect();
    }
    // must use web-sys to have interactive selection choise
    fn get_index(&self, row:u32, column:u32) -> usize {
		(row * self.width + column) as usize
	}


	//this section to defnie color, using blockchain interactivity
	#[near_bindgen]
	#[derive(Default, BorshDeserialize, BorshSerialize)]
	pub struct Counter {
	    val: i8, // i8 is signed. unsigned integers are also available: u8, u16, u32, u64, u128
	}

	#[near_bindgen]
	impl Counter {
	    // returns 8-bit signed integer, must match the type from our struct's 'val' defined above
	    pub fn get_index(&mut self) -> u8 {
	        .expect("Failed to retrieve number.")
	        return self.val;
	    }

	    // increment the counter
	    pub fn Change_color(&mut self) {
	        // note: adding one like this is an easy way to accidentally overflow
	        // real smart contracts will want to have safety checks
	        self.val = self.val + 1;
	        let log_message = format!("Increased number to {}", self.val);
	        env::log(log_message.as_bytes());
	        after_counter_change();
	        .expect("Intert positive number 0-255")
	    }

	    // decrement (subtract from) the counter
	    pub fn decrement(&mut self) {
	        // note: subtracting one like this is an easy way to accidentally overflow
	        // real smart contracts will want to have safety checks
	        self.val = self.val - 1;
	        let log_message = format!("Decreased number to {}", self.val);
	        env::log(log_message.as_bytes());
	        after_counter_change();
	    }
	}

// unlike the struct's functions above, this function cannot use attributes #[derive(…)] or #[near_bindgen]
// any attempts will throw helpful warnings upon 'cargo build'
// while this function cannot be invoked directly on the blockchain, it can be called from an invoked function
pub fn after_counter_change() {
    // show helpful warning that i8 (8-bit signed integer) will overflow above 127 or below -128
    env::log("Make sure you don't overflow, my friend.".as_bytes());
}
	// this function is for updating canvas each time a cell is updated
	// needs adjusted for multiplayer 
	// pub fn tick(&mut self) {
 //        let mut next = self.pixels.clone();

 //        for row in 0..self.height {
 //            for col in 0..self.width {
 //                let idx = self.get_index(row, col);
 //                let pix = self.cells[idx];
                
 //                // needs interactivity here
 //                // also define rules 
 //                let live_neighbors = self.live_neighbor_count(row, col);

 //                let next_cell = match (cell, live_neighbors) {
 //                    // Rule 1: Any live cell with fewer than two live neighbours
 //                    // dies, as if caused by underpopulation.
 //                    (Cell::Alive, x) if x < 2 => Cell::Dead,
 //                    // Rule 2: Any live cell with two or three live neighbours
 //                    // lives on to the next generation.
 //                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
 //                    // Rule 3: Any live cell with more than three live
 //                    // neighbours dies, as if by overpopulation.
 //                    (Cell::Alive, x) if x > 3 => Cell::Dead,
 //                    // Rule 4: Any dead cell with exactly three live neighbours
 //                    // becomes a live cell, as if by reproduction.
 //                    (Cell::Dead, 3) => Cell::Alive,
 //                    // All other cells remain in the same state.
 //                    (otherwise, _) => otherwise,
 //                };

 //                next[idx] = next_cell;
 //            }
 //        }

 //        self.cells = next;
 //    }

	pub fn new() -> Universe {
	        let width = 150;
	        let height = 150;
	        let rand = js_sys::Math::random();
	        let pixels = for i in range (0..width * height) {
	        	if rand > 0 && rand < 1 {Pixel::Red}
	        	else if {rand > 0 && rand < 1 {Pixel::Red}
	        	else if {rand > .1 && rand < .2 {Pixel::Orange}
	        	else if {rand > .2 && rand < .3 {Pixel::Yellow}
	        	else if {rand > .3 && rand < .4 {Pixel::Green}
	        	else if {rand > .4 && rand < .5 {Pixel::Blue}
	        	else if {rand > .5 && rand < .6 {Pixel::Purple}
	        	else if {rand > .6 && rand < .7 {Pixel::Pink}
	        	else if {rand > .7 && rand < .8 {Pixel::Brown}
	        	else if {rand > .8 && rand < .9 {Pixel::Black}
	        	else if {rand > .9 && rand < .99 {Pixel::White}
	        }
	        .collect();

	        let rand = js_sys::Math::random();
		    let pixels = (0..width*height)
		    	.map(|i| {
			        rand >0 && rand <.1 => { Pixel::Red} },
			        rand >.1 && rand <.2 => {Pixel::Orange},
			        rand >.2 && rand <.3 => {Pixel::Yellow},
			        rand >.3 && rand <.4 => { {Pixel::Green}},
			        rand >.4 && rand <.5 => { {Pixel::Blue} },
					rand >.5 && rand <.6 => {Pixel::Purple},
					rand >.6 && rand <.7 => {Pixel::Pink},
					rand >.7 && rand <.8 => {Pixel::Brown},
					rand >.8 && rand <.9 => {Pixel::Black},
					rand >.9 && rand <.99 => {Pixel::White},
			        _ => {Pixel::White}
	    		})
	            .collect();

	        Canvas {
	            width,
	            height,
	          	pixels,
	        }
	    }

	    pub fn render(&self) -> String {
	        self.to_string()
	    }
	}

}















