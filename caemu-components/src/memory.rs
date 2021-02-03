use caemu::component::{Component, In, InBus, InOutBus};
use caemu::bus::{Bus, Signal};
use caemu::delay::Delay;

use std::rc::Rc;
use std::cell::RefCell;

pub struct Ram32KB {
    a: InBus,
    d: InOutBus,
    ce: In,
    oe: In,
    we: In,
    mem: [u8; 1 << 15]
}

impl Ram32KB {
    pub fn new () -> Rc<RefCell<Ram32KB>> {
        Rc::new(RefCell::new(Ram32KB{
            a: InBus::new(&[10, 9, 8, 7, 6, 5, 4, 3, 25, 24, 21, 23, 2, 26, 1]),
            d: InOutBus::new(&[11, 12, 13, 15, 16, 17, 18, 19]),
            ce: In::new(20),
            oe: In::new(22),
            we: In::new(27),
            mem: [0; 1 << 15]}))
    }
}

impl Component for Ram32KB {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.a.connect(bus.clone());
        self.d.connect(bus.clone());
        self.ce.connect(bus.clone());
        self.oe.connect(bus.clone());
        self.we.connect(bus);
    }

    fn eval(&mut self) -> Delay {
        if self.ce.get() == Signal::ONE {
            self.d.set_high();
            return Delay::from_nanos(20);
        }

        if self.we.get() == Signal::ZERO {
            let addr = self.a.get_u16() as usize;
            self.mem[addr] = self.d.get_u8();
        }

        if self.oe.get() == Signal::ONE {
            self.d.set_high();
            return Delay::from_nanos(20);
        } else {
            let addr = self.a.get_u16() as usize;
            self.d.set_u8(self.mem[addr]);
            return Delay::from_nanos(55);
        }
    }
}