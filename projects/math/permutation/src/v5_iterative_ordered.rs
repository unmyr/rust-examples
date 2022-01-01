pub fn gen_perm<T>(v: Vec<T>)
-> Vec<Vec<T>>
where T: Clone + std::cmp::PartialOrd
{
    let num_of_chars = v.len();
    let mut result = Vec::<Vec<T>>::new();
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
    result.sort_by(|a, b| {
        let m = a.len() - 1;
        for i in 0 .. m {
            if a[i] != b[i] {
                return a[i].partial_cmp(&b[i]).unwrap();
            }
        }
        return a[m].partial_cmp(&b[m]).unwrap();
    });
    result
}

#[cfg(test)]
mod tests {
    use crate::v5_iterative_ordered::gen_perm;

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
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);
    }

    #[test]
    fn test_gen_perm_4() {
        let result = gen_perm(vec![1, 2, 3, 4]);
        println!("{:?}", result);
        assert_eq!(result, vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 4, 3],
            vec![1, 3, 2, 4],
            vec![1, 3, 4, 2],
            vec![1, 4, 2, 3],
            vec![1, 4, 3, 2],
            vec![2, 1, 3, 4],
            vec![2, 1, 4, 3],
            vec![2, 3, 1, 4],
            vec![2, 3, 4, 1],
            vec![2, 4, 1, 3],
            vec![2, 4, 3, 1],
            vec![3, 1, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 2, 1, 4],
            vec![3, 2, 4, 1],
            vec![3, 4, 1, 2],
            vec![3, 4, 2, 1],
            vec![4, 1, 2, 3],
            vec![4, 1, 3, 2],
            vec![4, 2, 1, 3],
            vec![4, 2, 3, 1],
            vec![4, 3, 1, 2],
            vec![4, 3, 2, 1],
        ]);
    }
}
