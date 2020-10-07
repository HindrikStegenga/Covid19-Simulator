use std::cmp::Ordering;

// By default floating point doesn't implement Ord since they don't have a total order relation.
// However, since we're guaranteed to not have NaN numbers, we do have a total order in our specific case.
// Therefore I implemented variants of f32 and f64 which are guaranteed to be non-NaN.

#[derive(PartialEq,PartialOrd, Copy, Clone)]
pub struct NonNanF64(pub f64);

impl NonNanF64 {
    pub fn new(val: f64) -> Option<NonNanF64> {
        if val.is_nan() {
            None
        } else {
            Some(NonNanF64(val))
        }
    }
}

impl Eq for NonNanF64 {}

impl Ord for NonNanF64 {
    fn cmp(&self, other: &NonNanF64) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialEq,PartialOrd, Copy, Clone)]
pub struct NonNanF32(pub f32);

impl NonNanF32 {
    pub fn new(val: f32) -> Option<NonNanF32> {
        if val.is_nan() {
            None
        } else {
            Some(NonNanF32(val))
        }
    }
}

impl Eq for NonNanF32 {}

impl Ord for NonNanF32 {
    fn cmp(&self, other: &NonNanF32) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}