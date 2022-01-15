use doubly_linked_list::v1::List;

fn main() {
    let mut list: List<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("{}", list);
}
