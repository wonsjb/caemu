use caemu::component::{Component, In, Out, InBus, OutBus, InOutBus};
use caemu::delay::Delay;
use caemu::bus::{Bus};
use caemu_macro::comp;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
struct State {
    pc: u16,
    sp: u8,
    acc: u8,
    x: u8,
    y: u8,
    ps: u8
}

impl State {
    fn new() -> Self {
        Self::default()
    }
}

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
    rst: In<40>,

    state: State
}

impl Component for CPU6502 {
    fn eval(&mut self) -> Delay {

        // Dummy impl
        self.state.acc += 1;
        self.state.pc += 1;
        self.state.x += 1;
        self.state.y += 1;
        self.state.sp += 1;
        self.state.ps += 1;

        Delay::from_micros(1)
    }
}
