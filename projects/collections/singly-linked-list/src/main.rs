use list::SinglyLinkedList;

fn main() {
    let mut list = SinglyLinkedList::new(1);
    list.push_back(2);
    list.push_back(3);
    println!("{}", list);
}
