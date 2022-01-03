use list::SinglyLinkedList;

fn main() {
    let mut list = SinglyLinkedList::new(1);
    list.push_back(2);
    list.push_back(3);
    println!("{}", list);
    assert_eq!(list.pop_back(), Some(3));
    println!("{}", list);
    assert_eq!(list.pop_back(), Some(2));
    println!("{}", list);
    assert_eq!(list.pop_back(), None);
    println!("{}", list);
}
