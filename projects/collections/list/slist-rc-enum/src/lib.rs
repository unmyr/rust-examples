use std::cmp::{PartialEq, PartialOrd};
use std::fmt::{Debug, Formatter, Result};
use std::rc::Rc;

#[derive(PartialOrd,PartialEq)]
pub enum SListNode<T> {
    Cons(T, Rc<SListNode<T>>),
    Nil,
}

impl<T: Debug> Debug for SListNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let SListNode::Cons(v, n) = self {
            write!(f, "SListNode({v:?}) {n:?}")
        } else {
            write!(f, "SListNode(Nil)")
        }
    }
}

/// A contiguous growable list type
pub struct SList<T> {
    node: Rc<SListNode<T>>,
}

impl<T>  SList<T> {
    /// # Examples
    ///
    /// ```
    /// use slist_rc_enum::SList;
    /// let mut list: SList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(
    ///     format!("{:?}", &list).as_str(),
    ///     "SList[SListNode(1), SListNode(2) SListNode(Nil)]"
    /// );
    /// ```
    pub fn push_back(&mut self, v: T) {
        let mut cur_ref = &mut self.node;
        while let Some(node) = Rc::get_mut(cur_ref) {
            cur_ref = match node {
                SListNode::Cons(_, next) => next,
                SListNode::Nil => {
                    *node = SListNode::Cons(v, Rc::new(SListNode::Nil));
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
    /// assert_eq!(
    ///     format!("{:?}", &list).as_str(),
    ///     "SList[SListNode(2), SListNode(1) SListNode(Nil)]"
    /// );
    /// ```
    pub fn push_front(&mut self, v: T) {
        let node: SListNode<T> = std::mem::replace(
            Rc::get_mut(&mut self.node).unwrap(),
            SListNode::Nil
        );
        self.node = Rc::new(
            SListNode::Cons(v, Rc::new(node))
        );
    }
}

impl<T> Default for SList<T> {
    fn default() -> Self {
        SList { node: Rc::new(SListNode::Nil)}
    }
}

impl<T: Debug> Debug for SList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if let SListNode::Cons(v, n) = self.node.as_ref() {
            write!(f, "SList[SListNode({v:?}), {n:?}]")
        } else {
            write!(f, "SList[]")
        }
    }
}

#[cfg(test)]
mod tests;