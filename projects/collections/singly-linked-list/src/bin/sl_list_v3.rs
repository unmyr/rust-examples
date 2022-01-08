use list::v3::SinglyLinkedList;

fn main() {
    let mut list = SinglyLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("{}", list);
}
