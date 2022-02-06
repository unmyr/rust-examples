use dlist_rc_opt_refcell::DList;

fn main() {
    let mut list: DList<u8> = Default::default();
    assert_eq!(format!("{:?}", list), "DList[]");

    list.push_front(3);
    assert_eq!(
        format!("{:?}", list),
        "DList[(3, prev:Nil, next:Nil)]"
    );

    list.push_front(2);
    assert_eq!(
        format!("{:?}", list),
        "DList[(value:2, prev:Nil, next:3) -> (value:3, prev:2, next:Nil)]"
    );

    list.push_front(1);
    assert_eq!(
        format!("{:?}", list),
        "DList[(value:1, prev:Nil, next:2) -> (value:2, prev:1, next:3) -> (value:3, prev:2, next:Nil)]"
    );

    println!("{list:?}");
}