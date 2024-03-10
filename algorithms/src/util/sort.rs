use std::cmp::Ordering;

pub fn descending<T: PartialOrd>(a: &T, b: &T) -> Ordering { 
    b.partial_cmp(a).unwrap_or(Ordering::Equal)
}