use super::*;

#[test]
fn test_insert() {
    let tree: BTree<&str> = Default::default();
    tree.insert("E");
    tree.insert("A");
    tree.insert("S");
    tree.insert("Y");
    assert_eq!(
        format!("{:?}", &tree),
        "BTree={TreeNode(Nil,\"A\",Nil), TreeNode(\"A\",\"E\",\"S\"), TreeNode(Nil,\"S\",\"Y\"), TreeNode(Nil,\"Y\",Nil)}"
    );
}

#[test]
fn test_to_vec_traversal_1_to_6() {
    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    assert_eq!(tree.to_vec_pre_order(), vec![4]);
    assert_eq!(tree.to_vec_pre_order_rc(), vec![4]);
    assert_eq!(tree.to_vec_in_order(), vec![4]);
    assert_eq!(tree.to_vec_in_order_rc(), vec![4]);
    assert_eq!(tree.to_vec_post_order_rc(), vec![4]);
    assert_eq!(tree.to_vec_post_order(), vec![4]);

    //      4
    //    2
    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(2);
    assert_eq!(tree.to_vec_pre_order(), vec![4, 2]);
    assert_eq!(tree.to_vec_pre_order_rc(), vec![4, 2]);
    assert_eq!(tree.to_vec_in_order(), vec![2, 4]);
    assert_eq!(tree.to_vec_in_order_rc(), vec![2, 4]);
    assert_eq!(tree.to_vec_post_order_rc(), vec![2, 4]);
    assert_eq!(tree.to_vec_post_order(), vec![2, 4]);

    //      4
    //        6
    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(6);
    assert_eq!(tree.to_vec_pre_order(), vec![4, 6]);
    assert_eq!(tree.to_vec_pre_order_rc(), vec![4, 6]);
    assert_eq!(tree.to_vec_in_order(), vec![4, 6]);
    assert_eq!(tree.to_vec_in_order_rc(), vec![4, 6]);
    assert_eq!(tree.to_vec_post_order_rc(), vec![6, 4]);
    assert_eq!(tree.to_vec_post_order(), vec![6, 4]);

    //      4
    //    2   6
    //  1    5
    let tree: BTree<u8> = Default::default();
    tree.insert(4);
    tree.insert(2);
    tree.insert(1);
    tree.insert(6);
    tree.insert(5);
    assert_eq!(tree.to_vec_pre_order(), vec![4, 2, 1, 6, 5]);
    assert_eq!(tree.to_vec_pre_order_rc(), vec![4, 2, 1, 6, 5]);
    assert_eq!(tree.to_vec_in_order(), vec![1, 2, 4, 5, 6]);
    assert_eq!(tree.to_vec_in_order_rc(), vec![1, 2, 4, 5, 6]);
    assert_eq!(tree.to_vec_post_order_rc(), vec![1, 2, 5, 6, 4]);
    assert_eq!(tree.to_vec_post_order(), vec![1, 2, 5, 6, 4]);
    assert_eq!(
        tree.iter_in_order().collect::<Vec<u8>>(),
        vec![1, 2, 4, 5, 6]
    );
}

#[test]
fn test_to_vec_traversal_a_to_i() {
    let tree: BTree<&str> = Default::default();
    tree.insert("F");
    tree.insert("B");
    tree.insert("G");
    tree.insert("A");
    tree.insert("D");
    tree.insert("C");
    tree.insert("E");
    tree.insert("I");
    tree.insert("H");

    assert_eq!(
        tree.to_vec_pre_order(),
        vec!["F", "B", "A", "D", "C", "E", "G", "I", "H"]
    );
    assert_eq!(
        tree.to_vec_pre_order_rc(),
        vec!["F", "B", "A", "D", "C", "E", "G", "I", "H"]
    );
    assert_eq!(
        tree.to_vec_in_order(),
        vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"]
    );
    assert_eq!(
        tree.to_vec_in_order_rc(),
        vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"]
    );
    assert_eq!(
        tree.iter_in_order().collect::<Vec<&str>>(),
        vec!["A", "B", "C", "D", "E", "F", "G", "H", "I"]
    );
    assert_eq!(
        tree.to_vec_post_order(),
        vec!["A", "C", "E", "D", "B", "H", "I", "G", "F"]
    );
    assert_eq!(
        tree.to_vec_post_order_rc(),
        vec!["A", "C", "E", "D", "B", "H", "I", "G", "F"]
    );
}
