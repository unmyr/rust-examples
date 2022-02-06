use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

pub enum SList<T> {
    Cons(T, Rc<SList<T>>),
    Nil,
}

impl<T> SList<T> {
    pub fn new(v: T, next: SList<T>) -> Self {
        SList::Cons(v, Rc::new(next))
    }

    fn is_nil(&self) -> bool {
        matches!(self, SList::Nil)
    }

    fn next_ref(&self) -> Option<&Rc<SList<T>>> {
        match self {
            SList::Nil => None,
            SList::Cons(_, next_rc_ref) => {
                Some(next_rc_ref)
            },
        }
    }

    fn next_ref_mut(&mut self) -> Option<&mut Rc<SList<T>>> {
        match self {
            SList::Nil => None,
            SList::Cons(_, next_rc_ref) => {
                Some(next_rc_ref)
            },
        }
    }

    /// # Examples
    ///
    /// ```
    /// use slist_rc_enum::SList;
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
        let mut cur_rc_ref_mut = self;

        while let SList::Cons(_, next_rc_ref_mut) = cur_rc_ref_mut {
            cur_rc_ref_mut = Rc::get_mut(next_rc_ref_mut).unwrap();
        }

        let _ = std::mem::replace(cur_rc_ref_mut, SList::from(v));
    }

    /// # Examples
    ///
    /// ```
    /// use slist_rc_enum::SList;
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

    /// # Examples
    ///
    /// ```
    /// use slist_rc_enum::SList;
    /// let mut list: SList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        let get_value = |n: SList<T>| {
            match n {
                SList::Nil => None,
                SList::Cons(v_ref, _) => Some(v_ref),
            }
        };
        let mut prev_rc_ref = match self {
            SList::Nil => return None,
            SList::Cons(_v_ref, next_rc_ref) => {
                if next_rc_ref.is_nil() {
                    // SList(x) -> SList(Nil)
                    // v
                    // SList(Nil)
                    return get_value(
                        std::mem::replace(self, SList::Nil)
                    );
                }
                next_rc_ref
            }
        };

        let tail_prev_rc_ref = loop {
            let is_prev_tail: bool = prev_rc_ref.next_ref().map(
                |next_ref| next_ref.is_nil()
            ).unwrap_or(false);
            if is_prev_tail { break prev_rc_ref }

            prev_rc_ref = Rc::get_mut(prev_rc_ref)?.next_ref_mut()?;
        };

        let tail_node: SList<T> = std::mem::replace(
            Rc::get_mut(tail_prev_rc_ref).unwrap(), SList::Nil
        );
        get_value(tail_node)
    }

    /// # Examples
    ///
    /// ```
    /// use slist_rc_enum::SList;
    /// let mut list: SList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        let head_rc_ref = self.next_ref_mut()?;

        let head_node: SList<T>;
        head_node = std::mem::replace(
            Rc::get_mut(head_rc_ref).unwrap(), SList::Nil
        );
        let head_node_old = std::mem::replace(self, head_node);
        match head_node_old {
            SList::Nil => None,
            SList::Cons(v_ref, _) => Some(v_ref)
        }
    }
}

impl<T> From<T> for SList<T> {
    fn from(v: T) -> Self { SList::new(v, SList::Nil) }
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