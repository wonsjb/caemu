use crate::bus::Signal;
use crate::delay::Delay;

use std::cell::Ref;
use std::time::Instant;

use std::collections::HashMap;

pub struct Logger {
    previous: Vec<Signal>
}

impl Logger {
    pub fn new(len: usize, names: Ref<HashMap<usize, String>>) -> Self {
        println!("$date {:?} $end", Instant::now());
        println!("$version caemu 0.0.1 $end");
        println!("$comment");
        println!("   Caemu simulation logger");
        println!("$end");
        println!("$timescale 1 ps $end");
        println!("$scope module caemu $end");
        let mut previous = Vec::new();
        for i in 0..len {
            let name_format = match names.get(&i) {
                None => format!("B{}", i),
                Some(name) => format!("{}", name)
            };
            println!("$var wire 1 B{} {} $end", i, name_format);
            previous.push(Signal::HIGH);
        }
        println!("$upscope $end");
        println!("$enddefinitions $end");

        Logger {previous}
    }

    pub fn log(&mut self, bus: &Vec<Signal>, current_time: &Delay) {
        let mut has_started = false;
        for (i, s) in bus.iter().enumerate() {
            if *s != self.previous[i] {
                let value = match s {
                    Signal::ONE => "1",
                    Signal::ZERO => "0",
                    Signal::HIGH => "x",
                };
                if !has_started {
                    print!("#{}", current_time.picoseconds);
                    has_started = true;
                }
                print!(" {}B{}", value, i);
                self.previous[i] = *s;
            }
        }
        if has_started {
            println!();
        }
    }
}