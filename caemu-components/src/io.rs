use caemu::component::{Component, In, InBus, Connect};
use caemu::bus::{Bus, Signal};
use caemu::delay::Delay;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Terminal {
    d: InBus,
    we: In,
    ce: In
}

impl Terminal {
    pub fn new () -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self{
            d: InBus::new(&[1, 2, 3, 4, 5, 6, 7, 8]),
            we: In::new(9),
            ce: In::new(10)}))
    }
}

impl Connect for Terminal {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.d.connect(bus.clone());
        self.we.connect(bus.clone());
        self.ce.connect(bus.clone());
    }
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
