use doubly_linked_list::v1::List;

fn main() {
    let mut list: List<u8> = Default::default();
    list.push_back(1);
    list.push_back(2);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(iter.next(), None);
    assert_eq!(list.iter().collect::<Vec<_>>(), vec![1]);

    list.push_back(2);
    list.push_back(3);
    println!("{}", list);
    assert_eq!(list.iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    for v in list.iter() {
        println!("{:?}", v);
    }
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);

    list.push_back(1);
    list.push_back(2);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.iter().collect::<Vec<_>>(), vec![2]);
}
