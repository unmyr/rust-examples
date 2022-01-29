use slist_rc_enum::SList;

fn main() {
    let mut list: SList<u8> = Default::default();
    println!("{:?}", &list);
    list.push_back(1);
    println!("{:?}", &list);
    list.push_back(2);
    println!("{:?}", &list);
    list.push_back(3);
    println!("{:?}", &list);
}