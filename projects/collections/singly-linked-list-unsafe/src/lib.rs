use std::default::Default;
use std::ptr::NonNull;
use std::fmt::{Display, Debug, Formatter, Result};

#[derive(Debug)]
pub struct ListNode<T: Debug> {
    value: T,
    next: Option<NonNull<ListNode<T>>>,
}

impl<T: Debug> ListNode<T> {
    pub fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: None }
    }
}

impl<T: Debug> Display for ListNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.next {
            Some(ref next) => {
                unsafe {
                    write!(f, "ListNode({:?}), {}", self.value, next.as_ref())
                }
            },
            None => write!(f, "ListNode({:?})", self.value)
        }
    }
}

#[derive(Default, Debug)]
pub struct SinglyLinkedList<T: Debug> {
    head: Option<NonNull<ListNode<T>>>,
}

impl<T: Clone + Debug> SinglyLinkedList<T> {
    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_unsafe::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// ```
    pub fn push_back(&mut self, v: T) {
        let node_new = NonNull::<ListNode<T>>::new(
            Box::into_raw(Box::new(ListNode::<T>::new(v)))
        );
        let mut cur: NonNull<ListNode<T>>;
        cur = match self.head {
            Some(cur) => cur,
            None => {
                self.head = node_new;
                return;
            }
        };

        unsafe {
            while let Some(next) = cur.as_ref().next {
                cur = next;
            }
            cur.as_mut().next = node_new;
        }
    }

    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_unsafe::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        let mut cur: NonNull<ListNode<T>>;
        cur = match self.head {
            Some(cur) => cur,
            None => return None,
        };

        let mut some_prev: Option<NonNull<ListNode<T>>> = None;
        while let Some(next) = unsafe { cur.as_ref().next } {
            some_prev = Some(cur);
            cur = next;
        }

        if let Some(mut prev) = some_prev {
            unsafe {
                println!(
                    "pop_back({:?}): prev(value:{:?}, next: {:?} => None)",
                    cur.as_ref().value,
                    prev.as_ref().value,
                    prev.as_ref().next
                );
                prev.as_mut().next = None;
            }
        } else {
            self.head = None;
        }

        let node : Box<ListNode<T>>;
        node = unsafe { Box::from_raw(cur.as_ptr()) };
        Some(node.value.clone())
    }

    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_unsafe::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        let head = match self.head {
            Some(head) => head,
            None => return None,
        };
        self.head = None;
        let node : Box<ListNode<T>> = unsafe {
            Box::from_raw(head.as_ptr())
        };
        self.head = node.next;
        let value = node.value.clone();
        Some(value)
    }

    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_unsafe::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> SinglyLinkedListIterator<T> {
        if let Some(head) = self.head {
            SinglyLinkedListIterator {
                cur: Some(head)
            }
        } else {
            SinglyLinkedListIterator { cur: None }
        }
    }
}

impl<T: Debug> Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self.head {
            Some(ref head) => {
                unsafe {
                    write!(f, "SinglyLinkedList[{}]", head.as_ref())
                }
            }
            None => write!(f, "SinglyLinkedList[]")
        }
    }
}

pub struct SinglyLinkedListIterator<T: Debug> {
    cur: Option<NonNull<ListNode<T>>>
}

impl<T: Clone + Debug> Iterator for SinglyLinkedListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = match self.cur {
            Some(cur) => cur,
            None => return None,
        };
        let cur_val: T;
        unsafe {
            cur_val = cur.as_ref().value.clone();
            if let Some(next) = cur.as_ref().next {
                self.cur = Some(next);
            } else {
                self.cur = None;
            }
        }
        Some(cur_val)
    }
}

impl<T: Debug> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        println!("> Dropping: SinglyLinkedList");
    }
}

impl<T: Debug> Drop for ListNode<T> {
    fn drop(&mut self) {
        println!("> Dropping: {:p}(value: {:?}, next: {:?})", self, self.value, self.next);
    }
}

#[cfg(test)]
mod tests;
