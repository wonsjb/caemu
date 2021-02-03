
#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Copy, Clone)]
pub struct Delay {
    pub picoseconds: u64
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
