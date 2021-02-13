use crate::board::{Board, CompleteBoard};
use crate::bus::{Signal, Bus};
use crate::component::{Component, In, Out, Connect};
use crate::delay::Delay;

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
        Rc::from(RefCell::from(ProbeInput{value: Signal::HIGH, out: Out::new(1)}))
    }

    pub fn set(&mut self, value: Signal) {
        self.value = value;
    }
}

impl Connect for ProbeInput {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.out.connect(bus);
    }

    fn get_name(&self, id: usize) -> String {
        format!("i{}", id)
    }
}

impl Component for ProbeInput {
    fn eval(&mut self) -> Delay {
        self.out.set(self.value);
        Delay::no_delay()
    }
}

impl ProbeOutput {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::from(RefCell::from(ProbeOutput{value: Signal::HIGH, input: In::new(1)}))
    }

    pub fn get(&self) -> Signal {
        self.value
    }
}

impl Connect for ProbeOutput {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.input.connect(bus);
    }

    fn get_name(&self, id: usize) -> String {
        format!("o{}", id)
    }
}

impl Component for ProbeOutput {


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

    pub fn from <T: Component + Connect + 'static> (
        inputs: &[usize],
        outputs: &[usize],
        component: Rc<RefCell<T>>,
        component_size: usize
    ) -> (Self, CompleteBoard) {
        // create the board
        let mut board = Board::new();

        // create the sockets on the board
        let mut socket_inputs = Vec::new();
        let mut socket_outputs = Vec::new();

        for i in 0..inputs.len() {
            let socket = board.socket(1);
            socket.pin(1).name(&format!("in{}", i));
            socket_inputs.push(socket);
        }

        for i in 0..outputs.len() {
            let socket = board.socket(1);
            socket.pin(1).name(&format!("out{}", i));
            socket_outputs.push(socket);
        }

        let component_socket = board.socket(component_size);

        // wire the socket together
        for (i, socket) in socket_inputs.iter_mut().enumerate() {
            socket.pin(1).connect(&component_socket.pin(inputs[i]));
        }
        for (i, socket) in socket_outputs.iter_mut().enumerate() {
            socket.pin(1).connect(&component_socket.pin(outputs[i]));
        }

        // Wire the board: no more connection allowed
        let mut board = board.wire();

        // plug component
        board.plug(component).into(component_socket);

        // Create tester + probes
        let tester = Tester::new(inputs.len(), outputs.len());
        for i in 0..inputs.len() {
            let input = tester.input(i);
            let input_socket = socket_inputs.remove(0);
            board.plug(input).into(input_socket);
        }
        for i in 0..outputs.len() {
            let output = tester.output(i);
            let output_socket = socket_outputs.remove(0);
            board.plug(output).into(output_socket);
        }

        (tester, board.complete())
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