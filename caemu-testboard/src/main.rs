use caemu::board::*;
use caemu::bus::*;
use caemu::delay::*;
use caemu_components::logics::*;
use caemu::tester::*;

// Test board, using a 6809 CPU, RAM (data), ROM (code) and a terminal

fn main() {
    // create the board
    let mut board = Board::new();

    // create the sockets on the board
    let socket_input1 = board.socket(1);
    let socket_input2 = board.socket(1);
    let socket_nand = board.socket(3);

    let socket_not_1 = board.socket(2);
    let socket_not_2 = board.socket(2);

    let socket_output = board.socket(1);

    // some naming
    socket_input1.pin(0).name("in1");
    socket_input2.pin(0).name("in2");
    socket_output.pin(0).name("out");

    // wire the socket together
    socket_input1.pin(0).connect(&socket_nand.pin(0));
    socket_input2.pin(0).connect(&socket_nand.pin(1));
    socket_nand.pin(2).connect(&socket_not_1.pin(0));
    socket_not_1.pin(1).connect(&socket_not_2.pin(0));
    socket_output.pin(0).connect(&socket_not_2.pin(1));

    // Wire the board: no more connection allowed
    let mut board = board.wire();

    // create components
    let nand = SN74LS00N::new();
    let not_1 = SN74LS04N::new();
    let not_2 = SN74LS04N::new();

    // Create probes
    let probe_input1 = ProbeInput::new();
    let probe_input2 = ProbeInput::new();
    let probe_output = ProbeOutput::new();

    // plug components / probes onto sockets
    board.plug(probe_input1.clone()).into(socket_input1);
    board.plug(probe_input2.clone()).into(socket_input2);
    board.plug(nand).into(socket_nand);
    board.plug(not_1).into(socket_not_1);
    board.plug(not_2).into(socket_not_2);
    board.plug(probe_output).into(socket_output);

    // complete the board, no plug allowed
    let mut board = board.complete();

    probe_input1.borrow_mut().set(Signal::ZERO);
    probe_input2.borrow_mut().set(Signal::ZERO);
    board.eval();

    board.move_time(Delay::from_nanos(1));

    probe_input1.borrow_mut().set(Signal::ONE);
    probe_input2.borrow_mut().set(Signal::ONE);
    board.eval();
}