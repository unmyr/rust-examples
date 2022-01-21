use singly_linked_list_using_box::v1::SinglyLinkedList;

fn main() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("{}", list);

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), None);
    println!("{}", list);

    list.push_back(1);
    list.push_back(2);
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}
