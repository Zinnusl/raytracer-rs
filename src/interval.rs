use contracts::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval<T> {
    pub left: T,
    pub right: T,
}

impl<T> Interval<T> {
    pub fn new(left: T, right: T) -> Interval<T> {
        Interval { left, right }
    }
}
