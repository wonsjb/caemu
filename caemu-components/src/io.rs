use caemu::component::{Component, In, InBus};
use caemu::bus::{Bus, Signal};
use caemu::delay::Delay;
use caemu_macro::comp;

use std::rc::Rc;
use std::cell::RefCell;

#[comp]
pub struct Terminal {
    d: In<1, 2, 3, 4, 5, 6, 7, 8>,
    we: In<9>,
    ce: In<10>
}

impl Component for Terminal {

    fn eval(&mut self) -> Delay {
        if self.ce.get() == Signal::ZERO && self.we.raised() {
            let data = self.d.get_u8() as char;
            print!("{}", data);
        }
        Delay::from_nanos(40)
    }
}
