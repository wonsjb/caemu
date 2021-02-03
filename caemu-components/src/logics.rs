use caemu::component::{Component, In, InBus, OutBus};
use caemu::delay::Delay;
use caemu::bus::{Bus};

use std::rc::Rc;
use std::cell::RefCell;

pub struct SN74LS00N {
    a: InBus,
    b: InBus,
    y: OutBus,
    gnd: In,
    vcc: In
}

impl SN74LS00N {
    pub fn new () -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self{
            a: InBus::new(&[1, 4, 10, 13]),
            b: InBus::new(&[2, 5, 9, 12]),
            y: OutBus::new(&[3, 6, 8, 11]),
            gnd: In::new(7),
            vcc: In::new(14)}))
    }
}

impl Component for SN74LS00N {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.a.connect(bus.clone());
        self.b.connect(bus.clone());
        self.y.connect(bus.clone());
        self.gnd.connect(bus.clone());
        self.vcc.connect(bus);
    }

    fn eval(&mut self) -> Delay {
        let a = self.a.get_u8();
        let b = self.b.get_u8();
        self.y.set_u8(!(a&b));
        Delay::from_nanos(15)
    }
}

pub struct SN74LS04N {
    a: InBus,
    y: OutBus,
    gnd: In,
    vcc: In
}

impl SN74LS04N {
    pub fn new () -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self{
            a: InBus::new(&[1, 3, 5, 9, 11, 13]),
            y: OutBus::new(&[2, 4, 6, 8, 10, 12]),
            gnd: In::new(7),
            vcc: In::new(14)}))
    }
}

impl Component for SN74LS04N {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.a.connect(bus.clone());
        self.y.connect(bus.clone());
        self.gnd.connect(bus.clone());
        self.vcc.connect(bus);
    }

    fn eval(&mut self) -> Delay {
        let a = self.a.get_u8();
        println!("a: {} not a: {}", a, !a);
        self.y.set_u8(!a);
        Delay::from_nanos(22)
    }
}


pub struct HC138 {
    a: InBus,
    e: InBus,
    y: OutBus,
    gnd: In,
    vcc: In
}

impl HC138 {
    pub fn new () -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self{
            a: InBus::new(&[1, 2, 3]),
            e: InBus::new(&[4, 5, 6]),
            y: OutBus::new(&[15, 14, 13, 12, 11, 10, 9, 7]),
            gnd: In::new(8),
            vcc: In::new(16)}))
    }
}

impl Component for HC138 {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.a.connect(bus.clone());
        self.e.connect(bus.clone());
        self.y.connect(bus.clone());
        self.gnd.connect(bus.clone());
        self.vcc.connect(bus);
    }

    fn eval(&mut self) -> Delay {
        let enable = self.e.get_u8();
        if enable != 4 {
            self.y.set_u8(0);
        } else {
            let a = self.a.get_u8();
            let y = 1 << a;
            self.y.set_u8(y);
        }
        Delay::from_nanos(53)
    }
}


#[cfg(test)]
mod tests {
    use caemu::tester::Tester;
    use caemu::bus::Signal;
    use crate::logics::*;

    #[test]
    fn nand_test() {
        // wrap a nand
        let (mut tester, mut board) = Tester::from(&[1, 2], &[3], SN74LS00N::new(), 14);

        // test few cases
        tester.test(&mut board, vec![Signal::ZERO, Signal::ZERO], vec![Signal::ONE]);
        tester.test(&mut board, vec![Signal::ZERO, Signal::ONE], vec![Signal::ONE]);
        tester.test(&mut board, vec![Signal::ONE, Signal::ZERO], vec![Signal::ONE]);
        tester.test(&mut board, vec![Signal::ONE, Signal::ONE], vec![Signal::ZERO]);
    }

    #[test]
    fn not_test() {
        // wrap a nand
        let (mut tester, mut board) = Tester::from(&[1], &[2], SN74LS04N::new(), 14);

        // test few cases
        tester.test(&mut board, vec![Signal::ZERO], vec![Signal::ONE]);
        tester.test(&mut board, vec![Signal::ONE], vec![Signal::ZERO]);
    }
}
