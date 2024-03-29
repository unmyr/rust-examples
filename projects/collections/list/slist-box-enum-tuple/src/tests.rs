use super::*;

#[test]
fn test_push_back() {
    let mut list: SList<u8> = Default::default();
    assert_eq!("SList(Nil)", format!("{:?}", &list).as_str());
    list.push_back(1);
    assert_eq!(
        "SList(1) -> SList(Nil)",
        format!("{:?}", &list).as_str()
    );
    list.push_back(2);
    assert_eq!(
        "SList(1) -> SList(2) -> SList(Nil)",
        format!("{:?}", &list).as_str()
    );
    list.push_back(3);
    assert_eq!(
        "SList(1) -> SList(2) -> SList(3) -> SList(Nil)",
        format!("{:?}", &list).as_str()
    );
}

#[test]
fn test_push_front() {
    let mut list: SList<u8> = Default::default();
    assert_eq!("SList(Nil)", format!("{:?}", &list).as_str());
    list.push_front(1);
    assert_eq!(
        "SList(1) -> SList(Nil)",
        format!("{:?}", &list).as_str()
    );
    list.push_front(2);
    assert_eq!(
        "SList(2) -> SList(1) -> SList(Nil)",
        format!("{:?}", &list).as_str()
    );
    list.push_front(3);
    assert_eq!(
        "SList(3) -> SList(2) -> SList(1) -> SList(Nil)",
        format!("{:?}", &list).as_str()
    );
}
