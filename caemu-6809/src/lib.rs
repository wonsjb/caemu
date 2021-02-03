use caemu::component::{Component, In, Out, OutBus, InOutBus};
use caemu::delay::Delay;
use caemu::bus::{Bus};

use std::rc::Rc;
use std::cell::RefCell;

pub struct CPU6809 {
    vss: In,
    nmi: In,
    irq: In,
    firq: In,
    bs: In,
    ba: In,
    vcc: In,
    a: OutBus,
    d: InOutBus,
    rw: Out,
    dma: Out,
    e: Out,
    q: Out,
    mrdy: Out,
    reset: In,
    extal: In,
    xtal: In,
    halt: In
}

impl CPU6809 {
    pub fn new () -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self{
            vss: In::new(0),
            nmi: In::new(1),
            irq: In::new(2),
            firq: In::new(3),
            bs: In::new(4),
            ba: In::new(5),
            vcc: In::new(6),
            a: OutBus::new(&[7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]),
            d: InOutBus::new(&[30, 29, 28, 27, 26, 25, 24, 23]),
            rw: Out::new(31),
            dma: Out::new(32),
            e: Out::new(33),
            q: Out::new(34),
            mrdy: Out::new(35),
            reset: In::new(36),
            extal: In::new(37),
            xtal: In::new(38),
            halt: In::new(39)
        }))
    }
}

impl Component for CPU6809 {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.vss.connect(bus.clone());
        self.nmi.connect(bus.clone());
        self.irq.connect(bus.clone());
        self.firq.connect(bus.clone());
        self.bs.connect(bus.clone());
        self.ba.connect(bus.clone());
        self.vcc.connect(bus.clone());
        self.a.connect(bus.clone());
        self.d.connect(bus.clone());
        self.rw.connect(bus.clone());
        self.dma.connect(bus.clone());
        self.e.connect(bus.clone());
        self.q.connect(bus.clone());
        self.mrdy.connect(bus.clone());
        self.reset.connect(bus.clone());
        self.extal.connect(bus.clone());
        self.xtal.connect(bus.clone());
        self.halt.connect(bus.clone());
    }

    fn eval(&mut self) -> Delay {
        Delay::from_picos(100)
    }
}
