use caemu::component::{Component, In, InBus, OutBus};
use caemu::delay::Delay;
use caemu::bus::{Bus};
use caemu_macro::comp;

use std::rc::Rc;
use std::cell::RefCell;

#[comp]
pub struct SN74LS00N {
    a: In<1, 4, 10, 13>,
    b: In<2, 5, 9, 12>,
    y: Out<3, 6, 8, 11>,
    gnd: In<7>,
    vcc: In<14>
}

impl Component for SN74LS00N {
    fn eval(&mut self) -> Delay {
        let a = self.a.get_u8();
        let b = self.b.get_u8();
        self.y.set_u8(!(a&b));
        Delay::from_nanos(15)
    }
}

#[comp]
pub struct SN74LS04N {
    a: In<1, 3, 5, 9, 11, 13>,
    y: Out<2, 4, 6, 8, 10, 12>,
    gnd: In<7>,
    vcc: In<14>
}

impl Component for SN74LS04N {
    fn eval(&mut self) -> Delay {
        let a = self.a.get_u8();
        println!("a: {} not a: {}", a, !a);
        self.y.set_u8(!a);
        Delay::from_nanos(22)
    }
}

#[comp]
pub struct HC138 {
    a: In<1, 2, 3>,
    e: In<5, 5, 6>,
    y: Out<15, 14, 13, 12, 11, 10, 9, 7>,
    gnd: In<8>,
    vcc: In<16>
}

impl Component for HC138 {

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
