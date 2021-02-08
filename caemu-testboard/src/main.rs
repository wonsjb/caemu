use caemu::board::*;
use caemu::delay::*;
use caemu_components::logics::*;
use caemu_components::memory::*;
use caemu_components::io::Terminal;

// Test board, using a 6809 CPU, RAM (data), ROM (code) and an output terminal

fn main() {
    // create the board
    let mut board = Board::new();

    let cpu = caemu_8bitcpus::cpu6502::CPU6502::new();
    let cpu_pins = cpu.borrow().get_pins();
    let rom = AT28C256::new();
    rom.borrow_mut().state.fill(&[1, 2, 3, 4]);

    let rom_pins = rom.borrow().get_pins();
    let ram = AS6C62256::new();
    let ram_pins = ram.borrow().get_pins();
    let demux = HC138::new();
    let terminal = Terminal::new();

    // create the sockets on the board
    let socket_rom = board.socket(28);
    let socket_ram = board.socket(28);
    let socket_cpu = board.socket(40);
    let socket_terminal = board.socket(10);
    let socket_demux = board.socket(16);

    // connections
    socket_cpu.pins(&cpu_pins.a)
        .connect(&socket_ram.pins(&ram_pins.a))
        .connect(&socket_rom.pins(&rom_pins.a));
    socket_cpu.pins(&cpu_pins.d)
        .connect(&socket_ram.pins(&ram_pins.d))
        .connect(&socket_rom.pins(&rom_pins.o));

    // Wire the board: no more connection allowed
    let mut board = board.wire();

    // plug components / probes onto sockets
    board.plug(cpu).into(socket_cpu);
    board.plug(ram).into(socket_ram);
    board.plug(rom).into(socket_rom);
    board.plug(demux).into(socket_demux);
    board.plug(terminal).into(socket_terminal);

    // complete the board, no plug allowed
    let mut board = board.complete();
    let mut time = Delay::no_delay();
    for _ in 0..50 {
        board.move_time(time);
        board.eval();
        time = time.plus(&Delay::from_micros(1));
    }
}