use crate::delay::Delay;
use crate::bus::{Bus, Signal};

use std::rc::Rc;
use std::cell::RefCell;

pub trait Component {
    fn eval(&mut self) -> Delay;

    fn connect(&mut self, bus: Rc<RefCell<Bus>>);
}


pub struct In {
    id: usize,
    bus: Option<Rc<RefCell<Bus>>>
}

pub struct InBus {
    inputs: Vec<In>
}

pub struct Out {
    id: usize,
    bus: Option<Rc<RefCell<Bus>>>
}

pub struct OutBus {
    outputs: Vec<Out>
}

pub struct InOut {
    id: usize,
    bus: Option<Rc<RefCell<Bus>>>
}

pub struct InOutBus {
    ios: Vec<InOut>
}

impl In {
    pub fn new(id: usize) -> Self {
        Self {id: id - 1, bus: None}
    }

    pub fn get(&self) -> Signal {
        match &self.bus {
            Some(bus) => bus.borrow().get(self.id),
            None => Signal::HIGH
        }      
    }

    pub fn raised(&self) -> bool {
        match &self.bus {
            Some(bus) => bus.borrow().raised(self.id),
            None => false
        }
    }

    pub fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.bus = Some(bus)
    }
}

impl InBus {
    pub fn new(ids: &[usize]) -> Self {
        let mut inputs = Vec::new();
        for id in ids {
            inputs.push(In::new(*id));
        }
        Self {inputs}
    }

    pub fn get(&self, id: usize) -> Signal {
        return self.inputs[id].get();
    }

    pub fn get_u8(&self) -> u8 {
        let mut res : u8 = 0;
        for i in 0..8 {
            if i < self.inputs.len() {
                if self.inputs[i].get() == Signal::ONE {
                    res = res | (1 << i);
                }
            } else {
                break;
            }
        }
        res
    }

    pub fn get_u16(&self) -> u16 {
        let mut res : u16 = 0;
        for i in 0..16 {
            if i < self.inputs.len() {
                if self.inputs[i].get() == Signal::ONE {
                    res = res | (1 << i);
                }
            } else {
                break;
            }
        }
        res
    }

    pub fn connect(&mut self, bus:Rc<RefCell<Bus>>) {
        for input in self.inputs.iter_mut() {
            input.connect(bus.clone());
        }
    }
}

impl Out {
    pub fn new(id: usize) -> Self {
        Self {id: id - 1, bus: None}
    }
    pub fn set(&mut self, signal: Signal) {
        if let Some(bus) = &self.bus {
            bus.borrow_mut().set(self.id, signal);
        }
    }

    pub fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.bus = Some(bus)
    }
}

impl InOut {
    pub fn new(id: usize) -> Self {
        Self {id : id - 1, bus: None}
    }

    pub fn get(&self) -> Signal {
        match &self.bus {
            Some(bus) => bus.borrow().get(self.id),
            None => Signal::HIGH
        }      
    }

    pub fn raised(&self) -> bool {
        match &self.bus {
            Some(bus) => bus.borrow().raised(self.id),
            None => false
        }
    }

    pub fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.bus = Some(bus)
    }

    pub fn set(&mut self, signal: Signal) {
        if let Some(bus) = &self.bus {
            bus.borrow_mut().set(self.id, signal);
        }
    }
}

impl InOutBus {
    pub fn new(ids: &[usize]) -> Self {
        let mut ios = Vec::new();
        for id in ids {
            ios.push(InOut::new(*id));
        }
        Self {ios}
    }

    pub fn get(&self, id: usize) -> Signal {
        return self.ios[id].get();
    }

    pub fn set(&mut self, id: usize, signal: Signal) {
        self.ios[id].set(signal);
    }

    pub fn set_high(&mut self) {
        for io in self.ios.iter_mut() {
            io.set(Signal::HIGH);
        }
    }

    pub fn set_u8(&mut self, data: u8) {
        for i in 0..8 {
            if i < self.ios.len() {
                if (data & (1 << i)) != 0{
                    self.ios[i].set(Signal::ONE);
                } else {
                    self.ios[i].set(Signal::ZERO);
                }
            } else {
                break;
            }
        }
    }

    pub fn get_u8(&self) -> u8 {
        let mut res : u8 = 0;
        for i in 0..8 {
            if i < self.ios.len() {
                if self.ios[i].get() == Signal::ONE {
                    res = res | (1 << i);
                }
            } else {
                break;
            }
        }
        res
    }

    pub fn connect(&mut self, bus:Rc<RefCell<Bus>>) {
        for io in self.ios.iter_mut() {
            io.connect(bus.clone());
        }
    }
}

impl OutBus {
    pub fn new(ids: &[usize]) -> Self {
        let mut outputs = Vec::new();
        for id in ids {
            outputs.push(Out::new(*id));
        }
        Self {outputs}
    }

    pub fn set(&mut self, id: usize, signal: Signal) {
        self.outputs[id].set(signal);
    }

    pub fn set_high(&mut self) {
        for io in self.outputs.iter_mut() {
            io.set(Signal::HIGH);
        }
    }

    pub fn set_u8(&mut self, data: u8) {
        for i in 0..8 {
            if i < self.outputs.len() {
                if (data & (1 << i)) != 0{
                    self.outputs[i].set(Signal::ONE);
                } else {
                    self.outputs[i].set(Signal::ZERO);
                }
            } else {
                break;
            }
        }
    }

    pub fn set_u16(&mut self, data: u16) {
        for i in 0..16 {
            if i < self.outputs.len() {
                if (data & (1 << i)) != 0{
                    self.outputs[i].set(Signal::ONE);
                } else {
                    self.outputs[i].set(Signal::ZERO);
                }
            } else {
                break;
            }
        }
    }

    pub fn connect(&mut self, bus:Rc<RefCell<Bus>>) {
        for output in self.outputs.iter_mut() {
            output.connect(bus.clone());
        }
    }
}