use caemu::component::{Component, In, InBus, OutBus};
use caemu::bus::{Bus, Signal};
use caemu::delay::Delay;

use std::rc::Rc;
use std::cell::RefCell;

pub struct SN74LS77 {
    d: InBus,
    c: InBus,
    q: OutBus,
    vcc: In,
    gnd: In,
    nc: InBus,

    mem: [Signal; 4]
}

impl SN74LS77 {
    pub fn new () -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self{
            d: InBus::new(&[1, 2, 5, 6]),
            c: InBus::new(&[12, 3]),
            q: OutBus::new(&[14, 13, 9, 8]),
            vcc: In::new(4),
            gnd: In::new(11),
            nc: InBus::new(&[7, 10]),
            mem: [Signal::ZERO; 4]}))
    }
}

impl Component for SN74LS77 {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.d.connect(bus.clone());
        self.c.connect(bus.clone());
        self.q.connect(bus.clone());
        self.vcc.connect(bus.clone());
        self.gnd.connect(bus.clone());
        self.nc.connect(bus);
    }

    fn eval(&mut self) -> Delay {
        if self.c.get(0) == Signal::ONE {
            self.q.set(0, self.d.get(0));
            self.q.set(1, self.d.get(1));
            self.mem[0] = self.d.get(0);
            self.mem[1] = self.d.get(1); 
            println!("Saving {:?} {:?}", self.mem[0], self.mem[1]);
        } else {
            println!("Loading {:?} {:?}", self.mem[0], self.mem[1]);
            self.q.set(0, self.mem[0]);
            self.q.set(1, self.mem[1]);
        }
        if self.c.get(1) == Signal::ONE {
            self.q.set(2, self.d.get(2));
            self.q.set(3, self.d.get(3));
            self.mem[2] = self.d.get(2);
            self.mem[3] = self.d.get(3); 
        } else {
            self.q.set(2, self.mem[2]);
            self.q.set(3, self.mem[3]);
        }
        Delay::from_nanos(40)
    }
}

#[cfg(test)]
mod tests {
    use caemu::tester::Tester;
    use crate::latches::*;

    #[test]
    fn latch_test() {
        // wrap a latch
        let (mut tester, mut board) = Tester::from(&[12, 1], &[14], SN74LS77::new(), 14);

        // test few cases
        tester.test(&mut board, vec![Signal::ONE, Signal::ZERO], vec![Signal::ZERO]);
        tester.test(&mut board, vec![Signal::ZERO, Signal::ZERO], vec![Signal::ZERO]);
        tester.test(&mut board, vec![Signal::ONE, Signal::ONE], vec![Signal::ONE]);
        tester.test(&mut board, vec![Signal::ZERO, Signal::ONE], vec![Signal::ONE]);
        tester.test(&mut board, vec![Signal::ZERO, Signal::ZERO], vec![Signal::ONE]);
    }
}
