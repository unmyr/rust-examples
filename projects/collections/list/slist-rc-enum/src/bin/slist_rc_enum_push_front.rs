use slist_rc_enum::SList;

fn main() {
    let mut list: SList<u8> = Default::default();
    assert_eq!(format!("{:?}", list), "SList(Nil)");

    list.push_front(3);
    assert_eq!(
        format!("{:?}", list),
        "SList(3) -> SList(Nil)"
    );

    list.push_front(2);
    assert_eq!(
        format!("{:?}", list),
        "SList(2) -> SList(3) -> SList(Nil)"
    );

    list.push_front(1);
    assert_eq!(
        format!("{:?}", list),
        "SList(1) -> SList(2) -> SList(3) -> SList(Nil)"
    );

    println!("{list:?}");
}