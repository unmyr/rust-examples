use singly_linked_list_refcell_opt_rc::SinglyLinkedList;

fn main() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    println!("{}", list);
}
