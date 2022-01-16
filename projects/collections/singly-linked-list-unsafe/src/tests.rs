use super::*;

#[test]
fn test_push_pop_1() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    list.push_back(1);
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
    list.push_back(1);
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn test_push_pop_2() {
    let mut list: SinglyLinkedList<&str> = Default::default();
    list.push_back("hello");
    list.push_back("world");
    assert_eq!(list.pop_back(), Some("world"));
    assert_eq!(list.pop_back(), Some("hello"));
    assert_eq!(list.pop_back(), None);
    list.push_back("hello");
    list.push_back("world");
    assert_eq!(list.pop_back(), Some("world"));
    assert_eq!(list.pop_back(), Some("hello"));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn test_pop_front_1() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    assert_eq!(list.pop_front(), None);

    list.push_back(1);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);

    list.push_back(1);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn test_pop_front_2() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), None);
}

#[ignore]
#[test]
fn test_iter_unwrap_failed() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(iter.next(), None);  // NG: Accessing the freed memory!

    list.push_back(2);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_last_add() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    list.push_back(1);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(1));
    list.push_back(2);
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(iter.next(), None);
}

#[ignore]
#[test]
fn test_iter_and_pop_front_1() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    let mut iter = list.iter();             // The next pointer points to 1.
    assert_eq!(list.pop_front(), Some(1));  // node 1 is dropped.
    assert_eq!(iter.next(), None);          // NG: Accessing the freed memory!
}

#[test]
fn test_iter_and_pop_front1() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    let mut iter = list.iter();            // The next pointer points to 1.
    assert_eq!(iter.next(), Some(1));      // The next pointer points to 2.
    assert_eq!(list.pop_front(), Some(1)); // node 1 is dropped.
    assert_eq!(iter.next(), Some(2));      // The next pointer points to None.
    assert_eq!(iter.next(), None);
}
