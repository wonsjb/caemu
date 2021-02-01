use caemu::board::{Component, Signal, Bus, Delay, In, Out};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Latch {
    clk: In,
    input: In,
    out: Out,

    mem: Signal
}

impl Latch {
    pub fn new () -> Rc<RefCell<Latch>> {
        Rc::new(RefCell::new(Latch{input: In::new(1), clk: In::new(0), out: Out::new(2), mem: Signal::ZERO}))
    }
}

impl Component for Latch {
    fn connect(&mut self, bus: Rc<RefCell<Bus>>) {
        self.input.connect(bus.clone());
        self.clk.connect(bus.clone());
        self.out.connect(bus);
    }

    fn eval(&mut self) -> Delay {
        if self.clk.raised() {
            self.mem = self.input.get();
        }
        self.out.set(self.mem);
        Delay::from_picos(40)
    }
}

#[cfg(test)]
mod tests {
    use caemu::board::{Board, Signal};
    use caemu::tester::Tester;
    use crate::latches::*;

    #[test]
    fn latch_test() {
        // create the board
        let mut board = Board::new();

        // create the sockets on the board
        let socket_input = board.socket(1);
        let socket_clock = board.socket(1);
        let socket_latch = board.socket(3);
        let socket_output = board.socket(1);

        socket_latch.pin(0).name("clk");
        socket_latch.pin(1).name("in");
        socket_latch.pin(2).name("out");

        // wire the socket together
        socket_clock.pin(0).connect(&socket_latch.pin(0));
        socket_input.pin(0).connect(&socket_latch.pin(1));
        socket_output.pin(0).connect(&socket_latch.pin(2));

        // Wire the board: no more connection allowed
        let mut board = board.wire();

        // create components
        let latch = Latch::new();

        // Create tester + probes
        let mut tester = Tester::new(2, 1);
        let probe_input = tester.input(0);
        let probe_clock = tester.input(1);
        let probe_output = tester.output(0);

        // plug components / probes onto sockets
        board.plug(probe_input).into(socket_input);
        board.plug(probe_clock).into(socket_clock);
        board.plug(probe_output).into(socket_output);
        board.plug(latch).into(socket_latch);

        // complete the board, no plug allowed
        let mut board = board.complete();

        // test few cases
        tester.test(&mut board, vec![Signal::ZERO, Signal::ZERO], vec![Signal::ZERO]);
        tester.test(&mut board, vec![Signal::ZERO, Signal::ONE], vec![Signal::ZERO]);
        tester.test(&mut board, vec![Signal::ONE, Signal::ZERO], vec![Signal::ZERO]);
        tester.test(&mut board, vec![Signal::ONE, Signal::ONE], vec![Signal::ONE]);
        tester.test(&mut board, vec![Signal::ZERO, Signal::ZERO], vec![Signal::ONE]);
    }
}
