use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

pub enum SList<T> {
    Cons(T, Rc<SList<T>>),
    Nil,
}

impl<T> SList<T> {
    pub fn new(v: T) -> Self {
        SList::Cons(v, Rc::new(SList::Nil))
    }

    /// # Examples
    ///
    /// ```
    /// use slist_rc_enum::SList;
    /// let mut list: SList<u8> = SList::new(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// assert_eq!(
    ///     format!("{:?}", &list).as_str(),
    ///     "SList(1) -> SList(2) -> SList(3) -> SList(Nil)"
    /// );
    /// ```
    pub fn push_back(&mut self, v: T) {
        let mut cur_ref = match self {
            SList::Nil => {
                let _ = std::mem::replace(
                    self,
                    SList::Cons(v, Rc::new(SList::Nil)) 
                );
                return;
            },
            SList::Cons(_, next) => next,
        };

        while let Some(node) = Rc::get_mut(cur_ref) {
            cur_ref = match node {
                SList::Cons(_, next) => next,
                SList::Nil => {
                    *node = SList::Cons(v, Rc::new(SList::Nil));
                    return;
                },
            };
        }
    }

    /// # Examples
    ///
    /// ```
    /// use slist_rc_enum::SList;
    /// let mut list: SList<u8> = SList::new(1);
    /// list.push_front(2);
    /// list.push_front(3);
    /// assert_eq!(
    ///     format!("{:?}", &list).as_str(),
    ///     "SList(3) -> SList(2) -> SList(1) -> SList(Nil)"
    /// );
    /// ```
    pub fn push_front(&mut self, v: T) {
        let head_node: SList<T> = std::mem::replace(
            self, SList::Nil
        );

        let _ = std::mem::replace(
            self, SList::Cons(v, Rc::new(head_node))
        );
    }
}

impl<T: Clone> SList<T> {
    /// # Examples
    ///
    /// ```
    /// use slist_rc_enum::SList;
    /// let mut list: SList<u8> = SList::new(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        let some_value: Option<T>;

        let next_rc_ref: &mut Rc<_> = match self {
            SList::Nil => return None,
            SList::Cons(v_ref, next_ref) => {
                some_value = Some(v_ref.clone());
                next_ref
            },
        };

        let next_node: SList<T> = std::mem::replace(
            Rc::get_mut(next_rc_ref).unwrap(), SList::Nil
        );
        let _ = std::mem::replace(self, next_node);
        some_value
    }
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