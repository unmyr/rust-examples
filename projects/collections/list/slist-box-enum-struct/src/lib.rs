use std::fmt::{Debug, Formatter, Result};

#[derive(std::clone::Clone)]
pub struct ConsCell<T> {
    value: T,
    next: Box<SList<T>>
}

impl<T: Debug> Debug for ConsCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let SList::Cons(cell) = self.next.as_ref() {
            write!(f, "SList({:?}) -> {:?}", self.value, cell.next)
        } else {
            write!(f, "SList({:?})", self.value)
        }
    }
}

#[derive(std::clone::Clone)]
pub enum SList<T> {
    Cons(ConsCell<T>),
    Nil,
}

impl<T: Clone> SList<T> {
    /// # Examples
    ///
    /// ```
    /// use slist_box_enum_struct::SList;
    /// let mut list: SList<u8> = Default::default();
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.push_front(3);
    /// assert_eq!(
    ///     format!("{:?}", &list).as_str(),
    ///     "SList(3) -> SList(2) -> SList(1) -> SList(Nil)"
    /// );
    /// ```
    pub fn push_front(&mut self, v: T) {
        match self {
            SList::Nil => {
                let _ = std::mem::replace(self, SList::from(v));
            },
            SList::Cons(head_cell_ref) => {
                let mut head_cell = ConsCell {
                    value: head_cell_ref.value.clone(),
                    next: Box::new(SList::Nil)
                };
                let _ = std::mem::replace(
                    &mut head_cell.next, head_cell_ref.next.clone()
                );
                let _ = std::mem::replace(
                    self, SList::Cons(ConsCell {
                        value: v,
                        next: Box::new(SList::Cons(head_cell))
                    })
                );
            },
        };
    }
}

impl<T> From<T> for SList<T> {
    fn from(v: T) -> Self {
        SList::Cons(
            ConsCell { value: v, next: Box::new(SList::Nil) }
        )
    }
}

impl<T> Default for SList<T> {
    fn default() -> Self { SList::Nil }
}

impl<T: Debug> Debug for SList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let SList::Cons(cell) = self {
            write!(f, "SList({:?}) -> {:?}", cell.value, cell.next)
        } else {
            write!(f, "SList(Nil)")
        }
    }
}

#[cfg(test)]
mod tests;
