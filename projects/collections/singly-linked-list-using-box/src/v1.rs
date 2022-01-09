use std::fmt;

#[derive(Debug)]
struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}

#[derive(Default, Debug)]
pub struct SinglyLinkedList<T> {
    head: Option<Box<ListNode<T>>>,
}

pub struct SinglyLinkedListIterator<'a, T:'a> {
    next: Option<&'a Box<ListNode<T>>>
}

impl<T: fmt::Debug> ListNode<T> {
    fn new(v: T) -> ListNode<T> {
        ListNode { value: v, next: None }
    }

    fn push_back(&mut self, v: T) {
        match &mut self.next {
            None => {
                self.next = Some(Box::new(ListNode::new(v)))
            },
            Some(ref mut next) => next.push_back(v),
        }
    }
}

impl<T: fmt::Debug> SinglyLinkedList<T> {
    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_using_box::v1::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// ```
    pub fn push_back(&mut self, v: T) {
        match &mut self.head {
            Some(head_ref) => head_ref.push_back(v),
            None => {
                let node_new = ListNode::new(v);
                self.head = Some(Box::new(node_new));
            }
        }
    }

    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_using_box::v1::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_back(), Some(2));
    /// assert_eq!(list.pop_back(), Some(1));
    /// assert_eq!(list.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        use std::mem::replace;
        let cur = replace(&mut self.head, None);
        cur.as_ref()?;

        let mut cur = cur.unwrap(); // safe because of the check above
        if cur.next.is_none() {
            return Some(cur.value);
        }

        let mut prev_next = &mut self.head;
        while cur.next.is_some() {
            // Take ownership of the next element
            let n_next = replace(&mut cur.next, None).unwrap();

            // Update the previous element's "next" field
            *prev_next = Some(cur);

            // Progress to the next element
            cur = n_next;

            // Progress our pointer to the previous element's "next" field
            prev_next = &mut prev_next.as_mut().unwrap().next;
        }

        Some(cur.value)
    }

    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_using_box::v1::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        use std::mem::replace;
        let cur = replace(&mut self.head, None);
        cur.as_ref()?;

        let cur = cur.unwrap();
        self.head = cur.next;
        Some(cur.value)
    }

    /// # Examples
    ///
    /// ```
    /// use singly_linked_list_using_box::v1::SinglyLinkedList;
    /// let mut list: SinglyLinkedList<u8> = Default::default();
    /// list.push_back(1);
    /// list.push_back(2);
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> SinglyLinkedListIterator<'_,T> {
        return SinglyLinkedListIterator {
            next: self.head.as_ref()
        }
    }
}

impl<T: fmt::Debug> fmt::Display for ListNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.next {
            Some(ref next) => {
                write!(f, "ListNode({:?}), {}", self.value, next)
            },
            None => write!(f, "ListNode({:?})", self.value)
        }
    }
}

impl<T: fmt::Debug> fmt::Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            Some(ref head) => write!(f, "SinglyLinkedList[{}]", head),
            None => write!(f, "SinglyLinkedList[]")
        }
    }
}

impl<'a, T: fmt::Debug> Iterator for SinglyLinkedListIterator<'a,T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| node);
            &node.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::SinglyLinkedList;

    #[test]
    fn test_pop_front() {
        let mut list: SinglyLinkedList<u8> = Default::default();
        assert_eq!(list.pop_front(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);

        list.push_back(1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

    }

    #[test]
    fn test_pop_back() {
        let mut list: SinglyLinkedList<u8> = Default::default();
        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_iter() {
        let mut list: SinglyLinkedList<u8> = Default::default();
        let mut iter = list.iter();
        assert_eq!(iter.next(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[ignore]
    fn test_iter_and_pop_front() {
        // let mut list: SinglyLinkedList<u8> = Default::default();
        // list.push_back(1);
        // list.push_back(2);
        // list.push_back(3);

        // let mut iter = list.iter();             // NG: immutable borrow occurs here
        // assert_eq!(list.pop_front(), Some(1));  // NG: mutable borrow occurs here
        // assert_eq!(iter.next(), None);          // NG: immutable borrow later used here
    }
}
