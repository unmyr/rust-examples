pub fn gen_perm_with_depth(v: Vec<u32>, m: usize, out: &mut Vec<Vec<u32>>)
{
    if m == v.len() {
        out.push(v.to_vec());
        return;
    }

    for i in m .. v.len() {
        let mut v_new = v.clone();
        if i != m {
            v_new.swap(m, i);
        }
        gen_perm_with_depth(v_new, m+1, out);
    }
}

#[cfg(test)]
mod tests {
    use crate::v2_recursive_unordered_u32::gen_perm_with_depth;

    #[test]
    fn test_gen_perm_1() {
        let mut result = Vec::<Vec<u32>>::new();
        gen_perm_with_depth(vec![1], 0, &mut result);
        println!("{:?}", result);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_gen_perm_2() {
        let mut result = Vec::<Vec<u32>>::new();
        gen_perm_with_depth(vec![1, 2], 0, &mut result);
        println!("{:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn test_gen_perm_3() {
        let mut result = Vec::<Vec<u32>>::new();
        gen_perm_with_depth(vec![1, 2, 3], 0, &mut result);
        println!("{:?}", result);
        assert_eq!(result, vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 2, 1],
            vec![3, 1, 2],
        ]);
    }

    #[test]
    fn test_gen_perm_4() {
        let mut result = Vec::<Vec<u32>>::new();
        gen_perm_with_depth(vec![1, 2, 3, 4], 0, &mut result);
        println!("{:?}", result);
        assert_eq!(result, vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 4, 3],
            vec![1, 3, 2, 4],
            vec![1, 3, 4, 2],
            vec![1, 4, 3, 2],
            vec![1, 4, 2, 3],
            vec![2, 1, 3, 4],
            vec![2, 1, 4, 3],
            vec![2, 3, 1, 4],
            vec![2, 3, 4, 1],
            vec![2, 4, 3, 1],
            vec![2, 4, 1, 3],
            vec![3, 2, 1, 4],
            vec![3, 2, 4, 1],
            vec![3, 1, 2, 4],
            vec![3, 1, 4, 2],
            vec![3, 4, 1, 2],
            vec![3, 4, 2, 1],
            vec![4, 2, 3, 1],
            vec![4, 2, 1, 3],
            vec![4, 3, 2, 1],
            vec![4, 3, 1, 2],
            vec![4, 1, 3, 2],
            vec![4, 1, 2, 3],
        ]);
    }
}
