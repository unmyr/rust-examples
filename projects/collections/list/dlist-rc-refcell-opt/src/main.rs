use dlist_rc_refcell_opt::DList;

fn main() {
    let mut list: DList<u8> = Default::default();
    assert_eq!(list.pop_front(), None);
    list.push_back(1);
    list.push_back(2);
    println!("{}", list);
}
