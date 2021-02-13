use std::rc::Rc;
use std::cell::{RefCell};
use crate::delay::Delay;
use crate::component::{Component, Connect};
use crate::bus::{Bus, Signal, IOAction};
use crate::logger::Logger;

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

pub struct Pins {
    ids: Vec<usize>,
    connections: Rc<RefCell<Vec<Connection>>>,
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

pub struct BoardComponent<'a, T: Component + Connect + 'static> {
    component: Rc<RefCell<T>>,
    board: &'a mut WiredBoard
}

impl <'a, T: Component + Connect> BoardComponent<'a, T> {
    pub fn into(self, mut socket: Socket) {
        socket.component = Some(self.component.clone());
        let mut inputs = Vec::new();
        let mut read = Vec::new();
        let mut output = Vec::new();
        for i in 0..socket.internal.size {
            inputs.push(self.board.id_to_wire[socket.location + i]);
            read.push(IOAction::None);
            output.push(IOAction::None);
            socket.pin(i+1).name(&self.component.borrow().get_name(i + 1));
        }
        let bus = Rc::from(RefCell::from(Bus{ids: inputs, read: RefCell::from(read), output,
            raised: self.board.raised.clone(),
            all_signals: self.board.all_signals.clone()}));
        self.component.borrow_mut().connect(bus.clone());
        self.board.components.push(WiredComponent{component: self.component, bus})
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
    pub fn plug<'a, T: Component + Connect>(&'a mut self, component: Rc<RefCell<T>>) -> BoardComponent<'a, T> {
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
        Pin {id: self.location + pin - 1, names: self.names.clone(), connections: self.connections.clone()}
    }

    pub fn pins(&self, pins: &[usize]) -> Pins {
        let mut pins_vec = Vec::new();
        for i in pins {
            pins_vec.push(self.location + i);
        }
        Pins {ids: pins_vec, connections: self.connections.clone()}
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

impl Pins {
    pub fn connect(&self, other: &Pins) -> &Self {
        for (from, to) in self.ids.iter().zip(&other.ids) {
            self.connections.borrow_mut().push(Connection{from: *from, to: *to});
        }
        self
    }  
}