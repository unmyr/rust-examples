use dlist_rc_refcell_opt::DList;

fn main() {
    let mut list: DList<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(
        format!("{:?}", list),
        "DList[(value:1, prev:Nil, next:2) -> (value:2, prev:1, next:3) -> (value:3, prev:2, next:Nil)]"
    );

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(
        format!("{:?}", list),
        "DList[(value:2, prev:Nil, next:3) -> (value:3, prev:2, next:Nil)]"
    );

    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(
        format!("{:?}", list),
        "DList[(3, prev:Nil, next:Nil)]"
    );

    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(format!("{:?}", list), "DList[]");

    assert_eq!(list.pop_front(), None);
    assert_eq!(format!("{:?}", list), "DList[]");
}