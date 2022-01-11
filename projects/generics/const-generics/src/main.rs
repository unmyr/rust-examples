use std::fmt::{Debug, Formatter, Result};

pub struct ArrayGenerics<T, const N: usize> {
    values: [T; N]
}

impl<T: Debug, const N: usize> ArrayGenerics<T, N> {
    pub fn new(values: [T; N]) -> Self {
        ArrayGenerics {
            values
        }
    }
}

impl<T: Debug, const N: usize> Debug for ArrayGenerics<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "ArrayGenerics({:?})", self.values)
    }
}

fn main() {
    let s = ArrayGenerics::new([1, 2, 3]);
    println!("{:?}", s);
}
