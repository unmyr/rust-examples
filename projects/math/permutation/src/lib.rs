#![feature(test)]
extern crate test;

pub fn perm_tmp_vec_u32(v1: Vec<u32>, v2: &mut Vec<u32>, out: &mut Vec<Vec<u32>>)
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
                perm_tmp_vec_u32(vc1, &mut vc2, out);
            }
        },
    }
}

pub fn perm_with_swap_u32(v: Vec<u32>, m: usize, out: &mut Vec<Vec<u32>>)
{
    if m == v.len() {
        out.push(v.to_vec());
        return;
    }

    for i in m .. v.len() {
        let mut v_new = v.clone();
        if i != m {
            let tmp = v_new[m];
            v_new[m] = v_new[i];
            v_new[i] = tmp;
        }
        perm_with_swap_u32(v_new, m+1, out);
    }
}

pub fn perm_with_swap_gen<T>(v: Vec<T>, m: usize, out: &mut Vec<Vec<T>>)
where T: Clone + std::cmp::PartialEq
{
    if m == v.len() {
        out.push(v.to_vec());
        return;
    }

    for i in m .. v.len() {
        let mut v_new = v.clone();
        if i != m {
            let tmp = v_new[m].clone();
            v_new[m] = v_new[i].clone();
            v_new[i] = tmp;
        }
        perm_with_swap_gen(v_new, m+1, out);
    }
}

pub fn perm_iterative_no_gen<T>(v: Vec<T>)
-> Vec<Vec<T>>
where T: Clone + std::cmp::PartialEq
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
                let tmp = v_new[n].clone();
                v_new[n] = v_new[i].clone();
                v_new[i] = tmp;
                result.push(v_new);
            }
        }
    }
    result
}

pub fn perm_iterative_ordered_gen<T>(v: Vec<T>)
-> Vec<Vec<T>>
where T: Clone + std::cmp::PartialEq + std::cmp::PartialOrd
{
    let num_of_chars = v.len();
    let mut result = Vec::<Vec<T>>::new();
    result.push(v);
    for n in 0 .. num_of_chars {
        let result_len = result.len();
        for result_idx in 0..(result_len) {
            for i in (n+1) .. num_of_chars {
                let mut v_new = result[result_idx].clone();
                let tmp = v_new[n].clone();
                v_new[n] = v_new[i].clone();
                v_new[i] = tmp;
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
    use test::Bencher;

    #[test]
    fn test_perm_tmp_vec_u32() {
        use crate::perm_tmp_vec_u32;

        let mut result = Vec::<Vec<u32>>::new();
        perm_tmp_vec_u32(vec![1, 2], &mut vec![], &mut result);
        println!("{:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn test_perm_with_swap_u32() {
        use crate::perm_with_swap_u32;

        let mut result = Vec::<Vec<u32>>::new();
        perm_with_swap_u32(vec![1, 2], 0, &mut result);
        println!("{:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn test_perm_with_swap_gen() {
        use crate::perm_with_swap_gen;

        let mut result = Vec::<Vec<u32>>::new();
        perm_with_swap_gen(vec![1, 2], 0, &mut result);
        println!("{:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn test_perm_iterative_no_gen() {
        use crate::perm_iterative_no_gen;

        let result = perm_iterative_no_gen(vec![1, 2]);
        println!("{:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn test_perm_iterative_ordered_gen() {
        use crate::perm_iterative_ordered_gen;

        let result = perm_iterative_ordered_gen(vec![1, 2]);
        println!("{:?}", result);
        assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
    }

    #[bench]
    fn bench_perm_tmp_vec_u32(b: &mut Bencher) {
        use crate::perm_tmp_vec_u32;
        
        b.iter(|| {
            for _ in 0..20 {
                // 6! = 720 patterns
                let mut result = Vec::<Vec<u32>>::new();
                perm_tmp_vec_u32(vec![1, 2, 3, 4, 5, 6], &mut vec![], &mut result);
                result.clear();
            }
        })
    }

    #[bench]
    fn bench_perm_with_swap_u32(b: &mut Bencher) {
        use crate::perm_with_swap_u32;
        
        b.iter(|| {
            for _ in 0..20 {
                // 6! = 720 patterns
                let mut result = Vec::<Vec<u32>>::new();
                perm_with_swap_u32(vec![1, 2, 3, 4, 5, 6], 0, &mut result);
                result.clear();
            }
        })
    }

    #[bench]
    fn bench_perm_with_swap_gen(b: &mut Bencher) {
        use crate::perm_with_swap_gen;
        
        b.iter(|| {
            for _ in 0..20 {
                // 6! = 720 patterns
                let mut result = Vec::<Vec<u32>>::new();
                perm_with_swap_gen(vec![1, 2, 3, 4, 5, 6], 0, &mut result);
                result.clear();
            }
        })
    }

    #[bench]
    fn bench_perm_iterative_no_gen(b: &mut Bencher) {
        use crate::perm_iterative_no_gen;
        
        b.iter(|| {
            for _ in 0..20 {
                // 6! = 720 patterns
                let mut result = perm_iterative_no_gen(vec![1, 2, 3, 4, 5, 6]);
                result.clear();
            }
        })
    }

    #[bench]
    fn bench_perm_iterative_ordered_gen(b: &mut Bencher) {
        use crate::perm_iterative_ordered_gen;
        
        b.iter(|| {
            for _ in 0..20 {
                // 6! = 720 patterns
                let mut result = perm_iterative_ordered_gen(vec![1, 2, 3, 4, 5, 6]);
                result.clear();
            }
        })
    }
}
