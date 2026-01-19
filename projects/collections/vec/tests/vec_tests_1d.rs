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

#[test]
fn it_vec_1d_max() {
    let v_i32 = vec![10, 5, 20, 15];
    let max = v_i32.iter().max().copied().unwrap();
    println!("max({:?})={}", &v_i32, max);
    assert_eq!(max, 20);

    let v_f32 = v_i32.iter().map(|v| *v as f32).collect::<Vec<f32>>();
    let max_idx = v_f32
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(idx, _)| idx);
    match max_idx {
        Some(max_idx) => {
            let max = v_f32[max_idx];
            assert_eq!(max, 20.);
            println!("max({:?})={}", &v_f32, max);
        }
        None => println!("The vector is empty!"),
    }
}
