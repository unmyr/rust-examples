pub fn gen_perm_with_tmp_vec_u32(v1: Vec<u32>, v2: &mut Vec<u32>, out: &mut Vec<Vec<u32>>)
{
    match v1.len() {
        0 => (),
        1 => {
            v2.push(v1[0]);
            out.push(v2.to_vec());
        },
        _ => {
            for some_x in &v1 {
                let mut vc1 = v1.clone();
                let mut vc2 = v2.clone();
                vc1.retain(|&cur| cur != *some_x);
                vc2.push(*some_x);
                gen_perm_with_tmp_vec_u32(vc1, &mut vc2, out);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::v1_recursive_ordered_u32::gen_perm_with_tmp_vec_u32;

    #[test]
    fn test_gen_perm_1() {
        let mut result = Vec::<Vec<u32>>::new();
        gen_perm_with_tmp_vec_u32(vec![1], &mut vec![], &mut result);
        println!("{:?}", result);
        assert_eq!(result, vec![vec![1]]);
    }

    #[test]
    fn test_gen_perm_2() {
        let mut result = Vec::<Vec<u32>>::new();
        gen_perm_with_tmp_vec_u32(vec![1, 2], &mut vec![], &mut result);
        println!("{:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn test_gen_perm_3() {
        let mut result = Vec::<Vec<u32>>::new();
        gen_perm_with_tmp_vec_u32(vec![1, 2, 3], &mut vec![], &mut result);
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
        let mut result = Vec::<Vec<u32>>::new();
        gen_perm_with_tmp_vec_u32(vec![1, 2, 3, 4], &mut vec![], &mut result);
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
