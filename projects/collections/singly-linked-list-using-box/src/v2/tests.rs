use super::*;

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
