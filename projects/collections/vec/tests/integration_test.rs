#[test]
fn it_vec_1d_iter() {
    let v1: Vec<i32> = vec![1, 2, 3];
    assert_eq!(v1.iter().map(|x| x + 1).collect::<Vec<_>>(), vec![2, 3, 4]);
}

// Sum using fold function on vec
#[test]
fn it_vec_1d_iter_sum_using_fold() {
    let v1: Vec<i32> = vec![1, 2, 3];
    println!(
        "ret={:?}",
        v1.iter().fold(0, |accumulator, &part| accumulator + part)
    );
    assert_eq!(
        v1.iter().fold(0, |accumulator, &part| accumulator + part),
        6
    );
    assert_eq!(v1, vec![1, 2, 3]);
}

// Sum using reduce function on vec
#[test]
fn it_vec_1d_iter_sum_using_reduce() {
    let v1: Vec<i32> = vec![1, 2, 3];
    assert_eq!(
        v1.iter()
            .cloned()
            .reduce(|accumulator, part| accumulator + part),
        Some(6)
    );
    assert_eq!(
        v1.into_iter()
            .reduce(|accumulator, part| accumulator + part),
        Some(6)
    );

    // empty vector
    let v2: Vec<i32> = Vec::new();
    assert_eq!(
        v2.into_iter()
            .reduce(|accumulator, part| accumulator + part),
        None
    );
}
