use std::rc::Rc;
use std::cell::{Ref, RefCell};

use std::time::Instant;

use petgraph::Graph;
use petgraph::algo::tarjan_scc;
use petgraph::prelude::*;

use std::collections::{HashMap, BTreeMap};

struct Connection {
    from: usize,
    to: usize
}

struct InternalSocket {
    size: usize
}

pub struct Board {
    sockets: Vec<Rc<InternalSocket>>,
    connections: Rc<RefCell<Vec<Connection>>>,
    current_count: usize,
    names: Rc<RefCell<HashMap<usize, String>>>
}

pub struct Socket {
    internal: Rc<InternalSocket>,
    component: Option<Rc<RefCell<dyn Component>>>,
    location: usize,
    connections: Rc<RefCell<Vec<Connection>>>,
    names: Rc<RefCell<HashMap<usize, String>>>
}

pub struct Pin {
    id: usize,
    connections: Rc<RefCell<Vec<Connection>>>,
    names: Rc<RefCell<HashMap<usize, String>>>
}

enum IOAction {
    None,
    IO(Signal)
}

pub struct Bus{
    ids: Vec<usize>,
    read: RefCell<Vec<IOAction>>,
    output: Vec<IOAction>,
    all_signals: Rc<RefCell<Vec<Signal>>>,
    raised: Rc<RefCell<Vec<bool>>>
}

impl Bus {

    pub fn get(&self, index: usize) -> Signal {
        let res = self.all_signals.borrow()[self.ids[index]];
        self.read.borrow_mut()[index] = IOAction::IO(res);
        res
    }

    pub fn raised(&self, index: usize) -> bool {
        let res = self.all_signals.borrow()[self.ids[index]];
        self.read.borrow_mut()[index] = IOAction::IO(res);
        return self.raised.borrow()[self.ids[index]];
    }

    pub fn set(&mut self, index: usize, signal: Signal) {
        self.output[index] = IOAction::IO(signal);
    }

    pub fn is_dirty(&self) -> bool {
        let all_signals = self.all_signals.borrow();
        for (pos, read) in self.read.borrow().iter().enumerate() {
            if let IOAction::IO(signal) = read {
                let signal_pos = self.ids[pos];
                if *signal != all_signals[signal_pos] {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn clear(&mut self) {
        for i in self.read.borrow_mut().iter_mut() {
            *i = IOAction::None
        }
        for i in self.output.iter_mut() {
            *i = IOAction::None
        }
    }

    pub fn apply(&mut self) {
        let mut all_signals = self.all_signals.borrow_mut();
        let mut raised = self.raised.borrow_mut();
        for (pos, id) in self.ids.iter().enumerate() {
            if let IOAction::IO(signal) = self.output[pos] {
                if all_signals[*id] == Signal::ZERO && signal == Signal::ONE {
                    raised[*id] = true;
                }
                all_signals[*id] = signal;
            }
        }
    }
}

struct WiredComponent {
    component: Rc<RefCell<dyn Component>>,
    bus: Rc<RefCell<Bus>>
}

pub struct WiredBoard {
    components: Vec<WiredComponent>,
    id_to_wire: Vec<usize>,
    all_signals: Rc<RefCell<Vec<Signal>>>,
    raised: Rc<RefCell<Vec<bool>>>,
    names: Rc<RefCell<HashMap<usize, String>>>
}

impl Board {
    pub fn new() -> Self {
        Board{
            sockets: Vec::new(),
            connections: Rc::from(RefCell::from(Vec::new())),
            names: Rc::from(RefCell::from(HashMap::new())),
            current_count: 0}
    }

    pub fn socket(&mut self, size: usize) -> Socket {
        let internal = Rc::new(InternalSocket{size});
        self.sockets.push(internal.clone());
        let res = Socket{internal,
            component: Option::None,
            location: self.current_count,
            names: self.names.clone(),
            connections: self.connections.clone()};
        self.current_count += size;
        res
    }

    pub fn wire(self) -> WiredBoard {
        let mut graph : Graph<usize, (), Undirected> = Graph::with_capacity(self.current_count, self.connections.borrow().len());
        let mut all_nodes = Vec::new();
        let mut names = HashMap::new();
        for i in 0..self.current_count {
            all_nodes.push(graph.add_node(i));
        }

        graph.extend_with_edges(self.connections.borrow().iter().map(|x| (all_nodes[x.from], all_nodes[x.to])));

        let connected = tarjan_scc(&graph);

        let mut id_to_wire = Vec::new();
        for _ in 0..self.current_count {
            id_to_wire.push(usize::MAX);
        }
        for (i, g) in connected.iter().enumerate() {
            for node in g {
                let id = *graph.node_weight(*node).unwrap();
                id_to_wire[id] = i;
                match self.names.borrow().get(&id) {
                    Some(name) => names.insert(i, name.clone()),
                    None => None
                };
            }
        }

        let mut all_signals = Vec::new();
        let mut raised = Vec::new();
        for _ in 0..connected.len() {
            all_signals.push(Signal::HIGH);
            raised.push(false);
        }
        WiredBoard{components: Vec::new(),
            id_to_wire,
            names: Rc::from(RefCell::from(names)),
            raised: Rc::from(RefCell::from(raised)),
            all_signals: Rc::from(RefCell::from(all_signals))}
    }
}

pub struct BoardComponent<'a> {
    component: Rc<RefCell<dyn Component>>,
    board: &'a mut WiredBoard
}

impl <'a> BoardComponent<'a> {
    pub fn into(self, mut socket: Socket) {
        socket.component = Some(self.component.clone());
        let mut inputs = Vec::new();
        let mut read = Vec::new();
        let mut output = Vec::new();
        for i in 0..socket.internal.size {
            inputs.push(self.board.id_to_wire[socket.location + i]);
            read.push(IOAction::None);
            output.push(IOAction::None);
        }
        let bus = Rc::from(RefCell::from(Bus{ids: inputs, read: RefCell::from(read), output,
            raised: self.board.raised.clone(),
            all_signals: self.board.all_signals.clone()}));
        self.component.borrow_mut().connect(bus.clone());
        self.board.components.push(WiredComponent{component: self.component, bus})
    }
}

pub struct Logger {
    previous: Vec<Signal>
}

impl Logger {
    pub fn new(len: usize, names: Ref<HashMap<usize, String>>) -> Self {
        println!("$date {:?} $end", Instant::now());
        println!("$version caemu 0.0.1 $end");
        println!("$comment");
        println!("   Caemu simulation logger");
        println!("$end");
        println!("$timescale 1 ps $end");
        println!("$scope module caemu $end");
        let mut previous = Vec::new();
        for i in 0..len {
            let name_format = match names.get(&i) {
                None => format!("B{}", i),
                Some(name) => format!("{}", name)
            };
            println!("$var wire 1 B{} {} $end", i, name_format);
            previous.push(Signal::HIGH);
        }
        println!("$upscope $end");
        println!("$enddefinitions $end");

        Logger {previous}
    }

    fn log(&mut self, bus: &Vec<Signal>, current_time: &Delay) {
        let mut has_started = false;
        for (i, s) in bus.iter().enumerate() {
            if *s != self.previous[i] {
                let value = match s {
                    Signal::ONE => "1",
                    Signal::ZERO => "0",
                    Signal::HIGH => "x",
                };
                if !has_started {
                    print!("#{}", current_time.picoseconds);
                    has_started = true;
                }
                print!(" {}B{}", value, i);
                self.previous[i] = *s;
            }
        }
        if has_started {
            println!();
        }
    }
}

pub struct CompleteBoard {
    components: Vec<WiredComponent>,
    all_signals: Rc<RefCell<Vec<Signal>>>,
    raised: Rc<RefCell<Vec<bool>>>,
    time: Delay,
    logger: Logger
}

fn get_empty_entry() -> Vec<usize> {
    vec!()
}

impl CompleteBoard {
    pub fn move_time(&mut self, time: Delay) {
        self.time = time;
    }

    pub fn eval(&mut self) {
        let mut current_time = self.time;
        let mut schedule : BTreeMap<Delay, Vec<usize>> = BTreeMap::new();

        for (i, c) in self.components.iter_mut().enumerate() {
            c.bus.borrow_mut().clear();
            let delay = c.component.borrow_mut().eval();
            let output_time = current_time.plus(&delay);
            let comp = schedule.entry(output_time).or_insert_with(get_empty_entry);
            comp.push(i);
        }

        loop {
            match schedule.iter().next() {
                Some(e) => {
                    current_time = *e.0;
                    for c in e.1 {
                        let component = self.components.get_mut(*c).unwrap();
                        component.bus.borrow_mut().apply();
                    }
                }
                None => break
            };
            schedule.remove(&current_time);
            self.logger.log(&self.all_signals.borrow(), &current_time);
            
            for (i, c) in self.components.iter_mut().enumerate() {
                if c.bus.borrow().is_dirty() {
                    let delay = c.component.borrow_mut().eval();
                    let output_time = current_time.plus(&delay);
                    schedule.entry(output_time).or_insert_with(get_empty_entry).push(i);
                }
            }
            for i in self.raised.borrow_mut().iter_mut() {
                *i = false;
            }
        }
        self.time = current_time;
    }
}

impl WiredBoard {
    pub fn plug<'a>(&'a mut self, component: Rc<RefCell<dyn Component>>) -> BoardComponent<'a> {
        BoardComponent{component, board: self}
    }

    pub fn complete(self) -> CompleteBoard {
        let len = self.all_signals.borrow().len();
        CompleteBoard {
            components: self.components,
            all_signals: self.all_signals,
            raised: self.raised,
            time: Delay::no_delay(),
            logger: Logger::new(len, self.names.borrow())
        }
    }
}

impl Socket {
    pub fn pin(&self, pin: usize) -> Pin {
        Pin {id: self.location + pin, names: self.names.clone(), connections: self.connections.clone()}
    }
}

impl Pin {
    pub fn connect(&self, other: &Pin) {
        self.connections.borrow_mut().push(Connection{from: self.id, to: other.id});
    }

    pub fn name(&self, name: &str) {
        self.names.borrow_mut().insert(self.id, String::from(name));
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Signal {
    ZERO,
    ONE,
    HIGH
}

pub trait Component {
    fn eval(&mut self) -> Delay;

    fn connect(&mut self, bus: Rc<RefCell<Bus>>);
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Copy, Clone)]
pub struct Delay {
    picoseconds: u64
}

impl Delay {

    pub fn plus(&self, other: &Self) -> Self {
        Delay { picoseconds: self.picoseconds + other.picoseconds }
    }

    pub fn no_delay() -> Self {
        Delay { picoseconds: 0 }
    }

    pub fn from_picos(pico: u64) -> Self {
        Delay { picoseconds: pico}       
    }

    pub fn from_nanos(nanos: u64) -> Self {
        Delay { picoseconds: nanos * 1000}       
    }

    pub fn from_micros(micros: u64) -> Self {
        Delay { picoseconds: micros * 1_000_000}       
    }

    pub fn from_millis(millis: u64) -> Self {
        Delay { picoseconds: millis * 1_000_000_000}       
    }

    pub fn from_seconds(seconds: u64) -> Self {
        Delay { picoseconds: seconds * 1_000_000_000_000}
    }
}

pub struct In {
    id: usize,
    bus: Option<Rc<RefCell<Bus>>>
}

pub struct Out {
    id: usize,
    bus: Option<Rc<RefCell<Bus>>>
}

impl In {
    pub fn new(id: usize) -> Self {
        In {id, bus: None}
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

impl Out {
    pub fn new(id: usize) -> Self {
        Out {id, bus: None}
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