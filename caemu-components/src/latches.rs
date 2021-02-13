use caemu::component::{Component, In, InBus, OutBus};
use caemu::bus::{Bus, Signal};
use caemu::delay::Delay;
use caemu_macro::comp;

use std::rc::Rc;
use std::cell::RefCell;

struct State {
    mem: [Signal; 4]
}

impl State {
    fn new() -> Self {
        Self {
            mem: [Signal::ZERO; 4]
        }
    }
}

#[comp]
pub struct SN74LS77 {
    d: In<1, 2, 5, 6>,
    c: In<12, 3>,
    q: Out<14, 13, 9, 8>,
    vcc: In<4>,
    gnd: In<11>,
    nc: In<7, 10>,

    state: State
}

impl Component for SN74LS77 {
    fn eval(&mut self) -> Delay {
        let state = &mut self.state;
        if self.c.get(0) == Signal::ONE {
            self.q.set(0, self.d.get(0));
            self.q.set(1, self.d.get(1));
            state.mem[0] = self.d.get(0);
            state.mem[1] = self.d.get(1); 
            println!("Saving {:?} {:?}", state.mem[0], state.mem[1]);
        } else {
            println!("Loading {:?} {:?}", state.mem[0], state.mem[1]);
            self.q.set(0, state.mem[0]);
            self.q.set(1, state.mem[1]);
        }
        if self.c.get(1) == Signal::ONE {
            self.q.set(2, self.d.get(2));
            self.q.set(3, self.d.get(3));
            state.mem[2] = self.d.get(2);
            state.mem[3] = self.d.get(3); 
        } else {
            self.q.set(2, state.mem[2]);
            self.q.set(3, state.mem[3]);
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
