use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Signal {
    ZERO,
    ONE,
    HIGH
}

pub enum IOAction {
    None,
    IO(Signal)
}

pub struct Bus{
    pub ids: Vec<usize>,
    pub read: RefCell<Vec<IOAction>>,
    pub output: Vec<IOAction>,
    pub all_signals: Rc<RefCell<Vec<Signal>>>,
    pub raised: Rc<RefCell<Vec<bool>>>
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