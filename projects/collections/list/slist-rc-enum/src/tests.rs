use super::*;

#[test]
fn test_push_pop_1() {
    let mut list: SList<u8> = Default::default();
    list.push_back(1);
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
    list.push_back(1);
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn test_push_pop_2() {
    let mut list: SList<&str> = Default::default();
    list.push_back("hello");
    list.push_back("world");
    assert_eq!(list.pop_back(), Some("world"));
    assert_eq!(list.pop_back(), Some("hello"));
    assert_eq!(list.pop_back(), None);
    list.push_back("hello");
    list.push_back("world");
    assert_eq!(list.pop_back(), Some("world"));
    assert_eq!(list.pop_back(), Some("hello"));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn test_push_back() {
    let mut list: SList<u8> = Default::default();
    assert_eq!(
        format!("{:?}", &list).as_str(), "SList(Nil)"
    );
    list.push_back(1);
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
    let mut list: SList<u8> = Default::default();
    assert_eq!(
        format!("{:?}", &list).as_str(), "SList(Nil)"
    );
    list.push_front(3);
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
    let mut list: SList<u8> = Default::default();
    list.push_back(1);
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
