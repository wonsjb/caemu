use caemu::component::{Component, In, InBus, InOutBus, OutBus};
use caemu::bus::{Bus, Signal};
use caemu::delay::Delay;
use caemu_macro::comp;

use std::rc::Rc;
use std::cell::RefCell;

pub struct StateRom {
    pub mem: [u8; 1 << 15]
}

impl StateRom {
    fn new() -> Self {
        StateRom {
            mem: [0; 1<<15]
        }
    }

    pub fn fill(&mut self, content: &[u8]) {
        for (i, byte) in content.iter().enumerate() {
            if i > self.mem.len() {
                break;
            }
            self.mem[i] = *byte;
        }
    }
}

// EEPROM 32k
#[comp]
pub struct AT28C256 {
    a: In<10, 9, 8, 7, 6, 5, 4, 3, 25, 24, 21, 23, 2, 26, 1>,
    o: Out<11, 12, 13, 15, 16, 17, 18, 19>,
    gnd: In<14>,
    ce: In<20>,
    oe: In<22>,
    we: In<27>,
    vcc: In<28>,

    pub state: StateRom
}

impl Component for AT28C256 {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.connect_impl(bus);
    }

    fn eval(&mut self) -> Delay {
        if self.ce.get() == Signal::ONE || self.oe.get() == Signal::ONE {
            self.o.set_high();
            return Delay::from_nanos(20);
        } else {
            let addr = self.a.get_u16() as usize;
            self.o.set_u8(self.state.mem[addr]);
            return Delay::from_nanos(150);
        }
    }
}

struct State {
    mem: [u8; 1 << 15]
}

impl State {
    fn new() -> Self {
        Self {
            mem: [0; 1 << 15]
        }
    }
}

// static ram 32k
#[comp]
pub struct AS6C62256 {
    a: In<10, 9, 8, 7, 6, 5, 4, 3, 25, 24, 21, 23, 2, 26, 1>,
    d: InOut<11, 12, 13, 15, 16, 17, 18, 19>,
    ce: In<20>,
    oe: In<22>,
    we: In<27>,
    vss: In<14>,
    vcc: In<28>,
    state: State
}

impl Component for AS6C62256 {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.connect_impl(bus);
    }

    fn eval(&mut self) -> Delay {
        if self.ce.get() == Signal::ONE {
            self.d.set_high();
            return Delay::from_nanos(20);
        }

        if self.we.get() == Signal::ZERO {
            let addr = self.a.get_u16() as usize;
            self.state.mem[addr] = self.d.get_u8();
        }

        if self.oe.get() == Signal::ONE {
            self.d.set_high();
            return Delay::from_nanos(20);
        } else {
            let addr = self.a.get_u16() as usize;
            self.d.set_u8(self.state.mem[addr]);
            return Delay::from_nanos(55);
        }
    }
}