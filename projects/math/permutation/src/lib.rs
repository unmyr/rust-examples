use std::ops::Range;

#![feature(test)]
extern crate test;

struct PermutationIterator<T> {
    initial: Vec<T>,
    ranges: Vec<Range<u16>>,
    indexes: Vec<u16>,
}

impl<T> PermutationIterator<T> {
    #![allow(dead_code)]
    pub fn new(p: Vec<T>) -> PermutationIterator<T>
    where T: Clone
    {
        let mut indexes = Vec::<u16>::with_capacity(p.len());
        let mut ranges = Vec::<Range<u16>>::with_capacity(p.len());
        indexes.push(0 as u16);
        ranges.push(0 as u16 .. 1 as u16);
        for i in 1 .. p.len() {
            ranges.push((i-1) as u16 .. p.len() as u16);
            indexes.push(ranges[i].start.clone());
        }
        PermutationIterator {
            initial: p.clone(),
            ranges: ranges,
            indexes: indexes,
        }
    }
}

impl<T: Clone> Iterator for PermutationIterator<T> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.indexes[0] > 0 {
            return None;
        }
        let mut v = self.initial.clone();
        let end = self.initial.len();
        for i in 0 .. (end-1) {
            if i != (self.indexes[i+1] as usize) {
                v.swap(i, self.indexes[i+1] as usize);
            }
        }
        self.indexes[end - 1] += 1;
        for i in (1 .. end).rev() {
            if self.indexes[i] >= self.ranges[i].end {
                self.indexes[i] = self.ranges[i].start;
                self.indexes[i - 1] += 1;
            }
        }
        return Some(v);
    }
}

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

    #[test]
    fn test_perm_iter_1() {
        use crate::PermutationIterator;

        let mut iter = PermutationIterator::new(
            vec![1 as u8]
        );
        assert_eq!(iter.next(), Some(vec![1]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_perm_iter_2() {
        use crate::PermutationIterator;

        let iter = PermutationIterator::new(
            vec![1 as u8, 2]
        );
        let result = iter.collect::<Vec<Vec<u8>>>();
        assert_eq!(result, vec![vec![1, 2], vec![2, 1]]);
    }

    #[test]
    fn test_perm_iter_3() {
        use crate::PermutationIterator;

        let mut iter = PermutationIterator::new(
            vec![1 as u8, 2, 3]
        );
        assert_eq!(iter.next(), Some(vec![1, 2, 3]));
        assert_eq!(iter.next(), Some(vec![1, 3, 2]));
        assert_eq!(iter.next(), Some(vec![2, 1, 3]));
        assert_eq!(iter.next(), Some(vec![2, 3, 1]));
        assert_eq!(iter.next(), Some(vec![3, 2, 1]));
        assert_eq!(iter.next(), Some(vec![3, 1, 2]));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_perm_iter_4() {
        use crate::PermutationIterator;

        let iter = PermutationIterator::new(
            vec![1 as u8, 2, 3, 4]
        );
        let mut verify = vec![
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
        ];
        assert_eq!(verify.len(), 24);
        for v in iter {
            if let Some(index) = verify.iter().position(|x| *x == v) {
                verify.remove(index);
            } else {
                assert!(verify.contains(&v), "{}", format!("{:?} not found.", &v));
            }
        }
        assert_eq!(verify.len(), 0);
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
