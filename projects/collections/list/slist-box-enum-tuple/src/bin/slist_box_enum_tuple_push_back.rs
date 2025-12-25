use slist_box_enum_tuple::SList;

fn main() {
    let mut list: SList<u8> = Default::default();
    assert_eq!(format!("{:?}", list), "SList(Nil)");

    list.push_back(1);
    assert_eq!(format!("{:?}", list), "SList(1) -> SList(Nil)");

    list.push_back(2);
    assert_eq!(format!("{:?}", list), "SList(1) -> SList(2) -> SList(Nil)");

    list.push_back(3);
    assert_eq!(
        format!("{:?}", list),
        "SList(1) -> SList(2) -> SList(3) -> SList(Nil)"
    );
}
