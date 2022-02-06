use std::fmt::{Debug, Formatter, Result};

pub enum SList<T> {
    Cons(T, Box<SList<T>>),
    Nil,
}

impl<T> SList<T> {
    fn new(v: T, next: SList<T>) -> Self {
        SList::Cons(v, Box::new(next))
    }

    /// # Examples
    ///
    /// ```
    /// use slist_box_enum_tuple::SList;
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
        let mut cur_slist_ref_mut = self;

        while let SList::Cons(_, next_boxed_ref_mut) = cur_slist_ref_mut {
            // &mut SList<T> <- &mut Box<SList<T>>
            cur_slist_ref_mut = next_boxed_ref_mut;
        }

        let _ = std::mem::replace(cur_slist_ref_mut, SList::from(v));
    }

    /// # Examples
    ///
    /// ```
    /// use slist_box_enum_tuple::SList;
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

        let _ = std::mem::replace(self, SList::new(v, head_node));
    }
}

impl<T> From<T> for SList<T> {
    fn from(v: T) -> Self {
        SList::new(v, SList::Nil)
    }
}

impl<T> Default for SList<T> {
    fn default() -> Self { SList::Nil }
}

impl<T: Debug> Debug for SList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let SList::Cons(v, n) = self {
            write!(f, "SList({v:?}) -> {n:?}")
        } else {
            write!(f, "SList(Nil)")
        }
    }
}

#[cfg(test)]
mod tests;
