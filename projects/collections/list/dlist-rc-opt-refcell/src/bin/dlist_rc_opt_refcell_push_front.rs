use dlist_rc_opt_refcell::DList;

fn main() {
    let mut list: DList<u8> = Default::default();
    println!("{list:?}"); list.push_front(3);
    println!("{list:?}"); list.push_front(2);
    println!("{list:?}"); list.push_front(1);
    println!("{list:?}");
}