use caemu::component::{Component, In, Out, OutBus, InOutBus};
use caemu::delay::Delay;
use caemu::bus::{Bus};
use caemu_macro::comp;

use std::rc::Rc;
use std::cell::RefCell;

#[comp]
pub struct CPU6809 {
    vss: In<1>,
    nmi: In<2>,
    irq: In<3>,
    firq: In<4>,
    bs: In<5>,
    ba: In<6>,
    vcc: In<7>,
    a: Out<8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23>,
    d: InOut<31, 30, 29, 28, 27, 26, 25, 24>,
    rw: Out<32>,
    dma: Out<33>,
    e: Out<34>,
    q: Out<35>,
    mrdy: Out<36>,
    reset: In<37>,
    extal: In<38>,
    xtal: In<39>,
    halt: In<40>
}

impl Component for CPU6809 {
    fn eval(&mut self) -> Delay {
        Delay::from_nanos(250)
    }
}
