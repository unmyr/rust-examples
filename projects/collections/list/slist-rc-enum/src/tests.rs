use super::*;

#[test]
fn test_push_back() {
    let mut list: SList<u8> = SList::new(1);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList(1) -> SList(Nil)"
    );
    list.push_back(2);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList(1) -> SList(2) -> SList(Nil)"
    );
    list.push_back(3);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList(1) -> SList(2) -> SList(3) -> SList(Nil)"
    );
}

#[test]
fn test_push_front() {
    let mut list: SList<u8> = SList::new(3);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList(3) -> SList(Nil)"
    );
    list.push_front(2);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList(2) -> SList(3) -> SList(Nil)"
    );
    list.push_front(1);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList(1) -> SList(2) -> SList(3) -> SList(Nil)"
    );
}

#[test]
fn test_pop_front() {
    let mut list: SList<u8> = SList::new(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), None);
    assert_eq!("SList(Nil)", format!("{:?}", &list).as_str());
    list.push_back(1);
    assert_eq!(
        "SList(1) -> SList(Nil)",
        format!("{:?}", &list).as_str(),
    );
}
