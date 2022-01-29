use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

pub enum SList<T> {
    Cons(T, Rc<SList<T>>),
    Nil,
}

impl<T> SList<T> {
    pub fn new() -> Self {
        SList::Nil
    }

    fn is_nil(&self) -> bool {
        match self {
            SList::Nil => true,
            _ => false,
        }
    }

    fn next_ref(&self) -> Option<&Rc<SList<T>>> {
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
        let mut cur_rc_ref = match self {
            SList::Nil => {
                let _ = std::mem::replace(
                    self, SList::Cons(v, Rc::new(SList::Nil)) 
                );
                return;
            },
            SList::Cons(_, next_rc_ref) => next_rc_ref,
        };

        while let Some(node_ref) = Rc::get_mut(cur_rc_ref) {
            cur_rc_ref = match node_ref {
                SList::Cons(_, next_rc_ref) => next_rc_ref,
                SList::Nil => {
                    *node_ref = SList::Cons(v, Rc::new(SList::Nil));
                    return;
                },
            };
        }
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

        let _ = std::mem::replace(
            self, SList::Cons(v, Rc::new(head_node))
        );
    }
}

impl<T: Clone> SList<T> {
    fn value(&self) -> Option<T> {
        match self {
            SList::Nil => None,
            SList::Cons(v_ref, _) => {
                Some(v_ref.clone())
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
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        let mut some_value: Option<T> = None;
        let mut cur_rc_ref = match self {
            SList::Nil => return None,
            SList::Cons(v_ref, next_rc_ref) => {
                if next_rc_ref.is_nil() {
                    some_value = Some(v_ref.clone());
                }
                next_rc_ref
            }
        };
        if some_value.is_some() {
            let _ = std::mem::replace(self, SList::Nil);
            return some_value;
        }

        match Rc::get_mut(cur_rc_ref).unwrap() {
            SList::Nil => return None,
            SList::Cons(v_ref, next_rc_ref) => {
                if next_rc_ref.is_nil() {
                    some_value = Some(v_ref.clone());
                }
            }
        };
        if some_value.is_some() {
            let _ = std::mem::replace(cur_rc_ref, Rc::new(SList::Nil));
            return some_value;
        }

        loop {
            let cur_value;
            let mut found = false;
            {
                cur_value = cur_rc_ref.value().unwrap();
                if let Some(next_ref) = cur_rc_ref.next_ref() {
                    if next_ref.next_ref().unwrap().is_nil() {
                        found = true;
                    }
                }
            }
            if found {
                let cur_node = std::mem::replace(
                    Rc::get_mut(cur_rc_ref).unwrap(),
                    SList::from(cur_value.clone())
                );
                return match cur_node {
                    SList::Nil => None,
                    SList::Cons(_v_ref, mut next_rc_ref) => {
                        return match Rc::get_mut(&mut next_rc_ref).unwrap() {
                            SList::Nil => None,
                            SList::Cons(v_ref, next_next_rc_ref) => {
                                assert!(next_next_rc_ref.is_nil());
                                Some(v_ref.clone())
                            },
                        };
                    },
                }
            }
            cur_rc_ref = match Rc::get_mut(cur_rc_ref).unwrap() {
                SList::Nil => return some_value,
                SList::Cons(_v_ref, next_rc_ref) => {
                    next_rc_ref
                }
            };
            some_value = Some(cur_value);
        }
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
        let some_value: Option<T>;

        let head_rc_ref: &mut Rc<_> = match self {
            SList::Nil => return None,
            SList::Cons(v_ref, head_rc_ref) => {
                some_value = Some(v_ref.clone());
                head_rc_ref
            },
        };

        let head_node: SList<T>;
        head_node = std::mem::replace(
            Rc::get_mut(head_rc_ref).unwrap(), SList::Nil
        );
        let _ = std::mem::replace(self, head_node);
        some_value
    }
}

impl<T> From<T> for SList<T> {
    fn from(v: T) -> Self {
        SList::Cons(v, Rc::new(SList::Nil))
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