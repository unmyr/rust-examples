use list::v4::SinglyLinkedList;

fn main() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("{}", list);
    assert_eq!(list.pop_back(), Some(3));
    println!("{}", list);
    assert_eq!(list.pop_back(), Some(2));
    println!("{}", list);
    assert_eq!(list.pop_back(), Some(1));
    println!("{}", list);
    assert_eq!(list.pop_back(), None);
    println!("{}", list);
}