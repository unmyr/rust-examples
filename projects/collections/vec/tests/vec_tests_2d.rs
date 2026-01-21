#[test]
fn it_vec_1d_to_2d() {
    let v1 = vec![1, 2, 3];
    let v2 = &v1
        .iter()
        .map(|v| vec![*v * -*v, *v * 2])
        .collect::<Vec<_>>();
    println!("{:?}", v2);
    println!("{:?} {:?}", v2[0][0], v2[0][1]);
    println!("{:?} {:?}", v2[1][0], v2[1][1]);
    assert_eq!(*v2, vec![[-1, 2], [-4, 4], [-9, 6]]);

    let v1 = vec![1, 2, 3];
    let v2 = &v1
        .iter()
        .cloned()
        .map(|v| vec![v * -v, v * 2])
        .collect::<Vec<_>>();
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?} {:?}", v2[0][0], v2[0][1]);
    println!("{:?} {:?}", v2[1][0], v2[1][1]);
    assert_eq!(*v2, vec![[-1, 2], [-4, 4], [-9, 6]]);

    let v1 = vec![1, 2, 3];
    let v2 = v1
        .into_iter()
        .map(|v| vec![v * -v, v * 2])
        .collect::<Vec<_>>();
    println!("{:?}", v2);
    println!("{:?} {:?}", v2[0][0], v2[0][1]);
    println!("{:?} {:?}", v2[1][0], v2[1][1]);
    assert_eq!(v2, vec![[-1, 2], [-4, 4], [-9, 6]]);
}
