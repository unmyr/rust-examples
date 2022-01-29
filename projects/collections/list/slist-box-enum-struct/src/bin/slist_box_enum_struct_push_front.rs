use slist_box_enum_struct::SList;

fn main() {
    let mut list: SList<u8> = Default::default();
    println!("{list:?}"); list.push_front(3);
    println!("{list:?}"); list.push_front(2);
    println!("{list:?}"); list.push_front(1);
    println!("{list:?}");
}