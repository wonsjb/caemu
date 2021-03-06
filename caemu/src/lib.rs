pub mod board;
pub mod tester;
pub mod component;
pub mod delay;
pub mod bus;
pub mod logger;

#[cfg(test)]
mod tests {
    use crate::delay::{Delay};

    #[test]
    fn delays_test() {
        assert_eq!(Delay::from_picos(1000), Delay::from_nanos(1));
        assert_eq!(Delay::from_nanos(1000), Delay::from_micros(1));
        assert_eq!(Delay::from_micros(1000), Delay::from_millis(1));
        assert_eq!(Delay::from_millis(1000), Delay::from_seconds(1));
    }
}