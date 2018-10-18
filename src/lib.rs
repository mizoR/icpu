extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

use std::fmt;
use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Led {
    Off = 0,
    On  = 1,
}

#[wasm_bindgen]
pub struct Cpu {
    leds:       Vec<Led>,
    tick_count: u32,
}

#[wasm_bindgen]
impl Cpu {
    pub fn new() -> Cpu {
        let leds = (0..8)
            .map(|_| { Led::Off })
            .collect();

        Cpu {
            leds:       leds,
            tick_count: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;

        let mut leds = self.leds.clone();

        let i = (self.tick_count / 10) % 16;

        leds[(i % 8) as usize] = if i / 8 > 0 { Led::Off } else { Led::On };

        self.leds = leds;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &led in self.leds.as_slice() {
            let symbol = if led == Led::Off { '◻' } else { '◼' };

            write!(f, "{}", symbol)?;
        }

        Ok(())
    }
}
