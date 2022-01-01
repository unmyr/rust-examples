pub mod v1_recursive_ordered_u32;
pub mod v2_recursive_unordered_u32;
pub mod v3_recursive_unordered_gen;
pub mod v4_iterative_unordered;
pub mod v5_iterative_ordered;

use std::ops::Range;

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

#[cfg(test)]
mod tests {
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
}
