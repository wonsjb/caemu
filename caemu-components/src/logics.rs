use caemu::board::{Component, Signal, Bus, Delay, In, Out};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Nand {
    in1: In,
    in2: In,
    out: Out
}

impl Nand {
    pub fn new () -> Rc<RefCell<Nand>> {
        Rc::new(RefCell::new(Nand{in1: In::new(0), in2: In::new(1), out: Out::new(2)}))
    }
}

impl Component for Nand {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.in1.connect(bus.clone());
        self.in2.connect(bus.clone());
        self.out.connect(bus);
    }

    fn eval(&mut self) -> Delay {
        self.out.set(match (self.in1.get(), self.in2.get()) {
                (Signal::ONE, Signal::ONE) => Signal::ZERO,
                (Signal::ZERO, _) | (_, Signal::ZERO) => Signal::ONE,
                _ => Signal::HIGH
        });
        Delay::from_picos(100)
    }
}

pub struct Not {
    input: In,
    out: Out
}

impl Not {
    pub fn new () -> Rc<RefCell<Not>> {
        Rc::new(RefCell::new(Not{input: In::new(0), out: Out::new(1)}))
    }
}

impl Component for Not {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.input.connect(bus.clone());
        self.out.connect(bus);
    }

    fn eval(&mut self) -> Delay {
        self.out.set(match self.input.get() {
                Signal::ONE => Signal::ZERO,
                Signal::ZERO => Signal::ONE,
                _ => Signal::HIGH
        });
        Delay::from_picos(40)
    }
}



#[cfg(test)]
mod tests {
    use caemu::board::{Board, Signal};
    use caemu::tester::Tester;
    use crate::logics::*;

    #[test]
    fn nand_test() {
        // create the board
        let mut board = Board::new();

        // create the sockets on the board
        let socket_input1 = board.socket(1);
        let socket_input2 = board.socket(1);
        let socket_nand = board.socket(3);

        let socket_not_1 = board.socket(2);
        let socket_not_2 = board.socket(2);

        let socket_output = board.socket(1);

        // wire the socket together
        socket_input1.pin(0).connect(&socket_nand.pin(0));
        socket_input2.pin(0).connect(&socket_nand.pin(1));
        socket_nand.pin(2).connect(&socket_not_1.pin(0));
        socket_not_1.pin(1).connect(&socket_not_2.pin(0));
        socket_output.pin(0).connect(&socket_not_2.pin(1));

        // Wire the board: no more connection allowed
        let mut board = board.wire();

        // create components
        let nand = Nand::new();
        let not_1 = Not::new();
        let not_2 = Not::new();

        // Create tester + probes
        let mut tester = Tester::new(2, 1);
        let probe_input1 = tester.input(0);
        let probe_input2 = tester.input(1);
        let probe_output = tester.output(0);

        // plug components / probes onto sockets
        board.plug(probe_input1).into(socket_input1);
        board.plug(probe_input2).into(socket_input2);
        board.plug(nand).into(socket_nand);
        board.plug(not_1).into(socket_not_1);
        board.plug(not_2).into(socket_not_2);
        board.plug(probe_output).into(socket_output);

        // complete the board, no plug allowed
        let mut board = board.complete();

        // test few cases
        tester.test(&mut board, vec![Signal::ZERO, Signal::ZERO], vec![Signal::ONE]);
        tester.test(&mut board, vec![Signal::ZERO, Signal::ONE], vec![Signal::ONE]);
        tester.test(&mut board, vec![Signal::ONE, Signal::ZERO], vec![Signal::ONE]);
        tester.test(&mut board, vec![Signal::ONE, Signal::ONE], vec![Signal::ZERO]);
    }
}
