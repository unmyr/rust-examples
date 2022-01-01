pub fn gen_perm<T>(v: Vec<T>)
-> Vec<Vec<T>>
where T: Clone
{
    let num_of_chars = v.len();
    let mut result = Vec::<Vec<T>>::new();
    // let vec_size = (1..=num_of_chars).fold(1, |acc, v| acc * v);
    // let mut out = Vec::<Vec<T>>::with_capacity(vec_size);
    result.push(v);
    for n in 0 .. num_of_chars {
        let result_len = result.len();
        for result_idx in 0..(result_len) {
            for i in (n+1) .. num_of_chars {
                let mut v_new = result[result_idx].clone();
                v_new.swap(n, i);
                result.push(v_new);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::v4_iterative_unordered::gen_perm;

    #[test]
    fn test_gen_perm_1() {
        let result = gen_perm(vec![1]);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_gen_perm_2() {
        let result = gen_perm(vec![1, 2]);
        assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn test_gen_perm_3() {
        let result = gen_perm(vec![1, 2, 3]);
        println!("{:?}", result);
        assert_eq!(result, vec![
            vec![1, 2, 3],
            vec![2, 1, 3],
            vec![3, 2, 1],
            vec![1, 3, 2],
            vec![2, 3, 1],
            vec![3, 1, 2],
        ]);
    }

    #[test]
    fn test_gen_perm_4() {
        let result = gen_perm(vec![1, 2, 3, 4]);
        println!("{:?}", result);
        assert_eq!(result, vec![
            vec![1, 2, 3, 4],
            vec![2, 1, 3, 4],
            vec![3, 2, 1, 4],
            vec![4, 2, 3, 1],
            vec![1, 3, 2, 4],
            vec![1, 4, 3, 2],
            vec![2, 3, 1, 4],
            vec![2, 4, 3, 1],
            vec![3, 1, 2, 4],
            vec![3, 4, 1, 2],
            vec![4, 3, 2, 1],
            vec![4, 1, 3, 2],
            vec![1, 2, 4, 3],
            vec![2, 1, 4, 3],
            vec![3, 2, 4, 1],
            vec![4, 2, 1, 3],
            vec![1, 3, 4, 2],
            vec![1, 4, 2, 3],
            vec![2, 3, 4, 1],
            vec![2, 4, 1, 3],
            vec![3, 1, 4, 2],
            vec![3, 4, 2, 1],
            vec![4, 3, 1, 2],
            vec![4, 1, 2, 3],
        ]);
    }
}