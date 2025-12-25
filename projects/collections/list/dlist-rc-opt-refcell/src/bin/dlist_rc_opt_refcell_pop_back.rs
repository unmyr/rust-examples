use dlist_rc_opt_refcell::DList;

fn main() {
    let mut list: DList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);
    println!("5:{list:?}");
    assert_eq!(list.pop_back(), Some(5));
    println!("4:{list:?}");
    assert_eq!(list.pop_back(), Some(4));
    println!("3:{list:?}");
    assert_eq!(list.pop_back(), Some(3));
    println!("2:{list:?}");
    assert_eq!(list.pop_back(), Some(2));
    println!("1:{list:?}");
    assert_eq!(list.pop_back(), Some(1));
    println!(" :{list:?}");
    assert_eq!(list.pop_back(), None);
}
