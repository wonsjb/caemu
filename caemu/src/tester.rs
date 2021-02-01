use crate::board::{CompleteBoard, Component, Signal, Bus, Delay, In, Out};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Tester {
    inputs: Vec<Rc<RefCell<ProbeInput>>>,
    outputs: Vec<Rc<RefCell<ProbeOutput>>>
}

pub struct ProbeInput {
    value: Signal,
    out: Out
}

pub struct ProbeOutput {
    value: Signal,
    input: In
}

impl ProbeInput {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::from(RefCell::from(ProbeInput{value: Signal::HIGH, out: Out::new(0)}))
    }

    pub fn set(&mut self, value: Signal) {
        self.value = value;
    }
}

impl Component for ProbeInput {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.out.connect(bus);
    }

    fn eval(&mut self) -> Delay {
        self.out.set(self.value);
        Delay::no_delay()
    }
}

impl ProbeOutput {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::from(RefCell::from(ProbeOutput{value: Signal::HIGH, input: In::new(0)}))
    }

    pub fn get(&self) -> Signal {
        self.value
    }
}

impl Component for ProbeOutput {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.input.connect(bus);
    }

    fn eval(&mut self) -> Delay {
        self.value = self.input.get();
        Delay::no_delay()
    }
}

impl Tester {
    pub fn new(inputs_count: usize, outputs_count: usize) -> Self {
        let mut inputs = Vec::new();
        for _ in 0..inputs_count {
            inputs.push(ProbeInput::new())
        }
        let mut outputs = Vec::new();
        for _ in 0..outputs_count {
            outputs.push(ProbeOutput::new())
        }
        Tester{inputs, outputs}
    }

    pub fn input(&self, input_id: usize) -> Rc<RefCell<ProbeInput>> {
        self.inputs.get(input_id).unwrap().clone()
    }

    pub fn output(&self, output_id: usize) -> Rc<RefCell<ProbeOutput>> {
        self.outputs.get(output_id).unwrap().clone()
    }

    pub fn test(&mut self, board: &mut CompleteBoard, inputs: Vec<Signal>, outputs: Vec<Signal>) {
        assert_eq!(inputs.len(), self.inputs.len());
        assert_eq!(outputs.len(), self.outputs.len());

        for iter in self.inputs.iter_mut().zip(inputs) {
            iter.0.borrow_mut().set(iter.1);
        }

        board.eval();

        for iter in self.outputs.iter().zip(outputs) {
            assert_eq!(iter.0.borrow().get(), iter.1);
        }
    }
}