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

impl<T> SList<T> {
    /// # Examples
    ///
    /// ```
    /// use slist_box_enum_struct::SList;
    /// let mut list: SList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// assert_eq!(
    ///     format!("{:?}", &list).as_str(),
    ///     "SList(1) -> SList(2) -> SList(3) -> SList(Nil)"
    /// );
    /// ```
    pub fn push_back(&mut self, v: T) {
        let mut cur_box_ref = self;

        while let SList::Cons(cons_cell_ref) = cur_box_ref {
            cur_box_ref = &mut *cons_cell_ref.next;
        }

        let _ = std::mem::replace(
            cur_box_ref,
            SList::Cons(
                ConsCell {
                    value: v,
                    next: Box::new(SList::Nil)
                }
            )
        );
    }

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
        let head_node: SList<T>;
        head_node = std::mem::replace(self, SList::Nil);

        let _ = std::mem::replace(
            self,
            SList::Cons(
                ConsCell {
                    value: v,
                    next: Box::new(head_node)
                }
            )
        );
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
