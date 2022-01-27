use super::*;

#[test]
fn test_push_back() {
    let mut list: SList<u8> = Default::default();
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList[]"
    );
    list.push_back(1);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList[SListNode(1), SListNode(Nil)]"
    );
    list.push_back(2);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList[SListNode(1), SListNode(2) SListNode(Nil)]"
    );
    list.push_back(3);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList[SListNode(1), SListNode(2) SListNode(3) SListNode(Nil)]"
    );
}

#[test]
fn test_push_front() {
    let mut list: SList<u8> = Default::default();
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList[]"
    );
    list.push_front(3);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList[SListNode(3), SListNode(Nil)]"
    );
    list.push_front(2);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList[SListNode(2), SListNode(3) SListNode(Nil)]"
    );
    list.push_front(1);
    assert_eq!(
        format!("{:?}", &list).as_str(),
        "SList[SListNode(1), SListNode(2) SListNode(3) SListNode(Nil)]"
    );
}
