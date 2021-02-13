use caemu::component::{Component, In, Out, InBus, OutBus, InOutBus};
use caemu::delay::Delay;
use caemu::bus::{Bus};
use caemu_macro::comp;

use std::rc::Rc;
use std::cell::RefCell;

#[comp]
pub struct CPU6502 {
    vss: In<1, 21>,
    rdy: In<2>,
    phy: In<37, 3, 39>,
    irq: In<4>,
    nc: In<5, 34>,
    nmi: In<6>,
    sync: In<7>,
    vcc: In<8>,
    a: Out<9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 23, 24, 25>,
    d: InOut<33, 32, 31, 30, 29, 28, 27, 26>,
    halt: In<35>,
    rw: Out<36>,
    s0: In<38>,
    rst: In<40>
}

impl Component for CPU6502 {
    fn eval(&mut self) -> Delay {
        Delay::from_micros(1)
    }
}
