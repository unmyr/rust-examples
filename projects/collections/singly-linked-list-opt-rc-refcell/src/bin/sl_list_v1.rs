use list::v1::SinglyLinkedList;

fn main() {
    let mut list: SinglyLinkedList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("{}", list);
}
