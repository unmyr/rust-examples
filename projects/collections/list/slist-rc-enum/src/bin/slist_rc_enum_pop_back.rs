use slist_rc_enum::SList;

fn main() {
    let mut list: SList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    println!("{list:?}");
    assert_eq!(list.pop_back(), Some(5));
    println!("{list:?}");
    assert_eq!(list.pop_back(), Some(4));
    println!("{list:?}");
    assert_eq!(list.pop_back(), Some(3));
    println!("{list:?}");
    assert_eq!(list.pop_back(), Some(2));
    println!("{list:?}");
    assert_eq!(list.pop_back(), Some(1));
    println!("{list:?}");
    assert_eq!(list.pop_back(), None);
}
