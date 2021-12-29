///
/// # Examples
/// ```
/// use necklace_permutations::necklace_perm_with_filter;
/// let result = necklace_perm_with_filter(vec![1, 2, 3, 4]);
/// assert_eq!(result.len(), 3);
/// assert!(result.contains(&vec![1, 2, 4, 3]));
/// assert!(result.contains(&vec![1, 2, 3, 4]));
/// assert!(result.contains(&vec![1, 3, 2, 4]));
/// ```
pub fn necklace_perm_with_filter<T>(v: Vec<T>)
-> Vec<Vec<T>>
where T: Clone + std::cmp::PartialOrd
{
    let num_of_chars = v.len();
    let mut result = Vec::<Vec<T>>::new();
    result.push(v);
    if num_of_chars <= 3 {
        return result;
    }

    for n in 1 .. num_of_chars {
        let result_len = result.len();
        for result_idx in 0..(result_len) {
            for i in (n+1) .. num_of_chars {
                let mut v_new = result[result_idx].clone();
                v_new.swap(n, i);
                result.push(v_new);
            }
        }
    }
    result.into_iter().filter(|r| r[1] < r[r.len()-1]).collect()
}

/// # Examples
/// ```
/// use necklace_permutations::necklace_perm;
/// let result = necklace_perm(vec![1, 2, 3, 4]);
/// assert_eq!(result.len(), 3);
/// assert!(result.contains(&vec![1, 2, 4, 3]));
/// assert!(result.contains(&vec![1, 2, 3, 4]));
/// assert!(result.contains(&vec![1, 3, 2, 4]));
/// ```
pub fn necklace_perm<T>(v: Vec<T>)
-> Vec<Vec<T>>
where T: Clone + std::cmp::PartialOrd
{
    let num_of_chars = v.len();
    let mut result = Vec::<Vec<T>>::new();
    result.push(v);
    if num_of_chars <= 3 {
        return result;
    }

    let end: usize = num_of_chars - 1;
    for i in 2 .. (num_of_chars-1) {
        let mut v_new = result[0].clone();
        v_new.swap(end, i);
        result.push(v_new);
    }

    let n: usize = 1;
    let result_len = result.len();
    for result_idx in 0..(result_len) {
        for i in (n+1) .. (num_of_chars-1) {
            if result[result_idx][i] > result[result_idx][end] {
                continue;
            }
            let mut v_new = result[result_idx].clone();
            v_new.swap(n, i);
            result.push(v_new);
        }
    }

    for n in 2 .. num_of_chars {
        let result_len = result.len();
        for result_idx in 0..(result_len) {
            for i in (n+1) .. (num_of_chars-1) {
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
    #[test]
    fn test_perm1_order_2() {
        use crate::necklace_perm_with_filter;
        let result = necklace_perm_with_filter(vec![1, 2]);
        assert_eq!(result.len(), 1);
        assert_eq!(result, vec![vec![1, 2]]);
    }

    #[test]
    fn test_perm1_order_4() {
        use crate::necklace_perm_with_filter;
        let result = necklace_perm_with_filter(vec![1, 2, 3, 4]);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&vec![1, 2, 4, 3]));
        assert!(result.contains(&vec![1, 2, 3, 4]));
        assert!(result.contains(&vec![1, 3, 2, 4]));
    }

    #[test]
    fn test_perm2_order_2() {
        use crate::necklace_perm;
        let result = necklace_perm(vec![1, 2]);
        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result, vec![vec![1, 2]]);
    }

    #[test]
    fn test_perm2_order_3() {
        use crate::necklace_perm;
        let result = necklace_perm(vec![1, 2, 3]);
        println!("{:?}", result);
        assert_eq!(result.len(), 1);
        assert_eq!(result, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_perm2_order_4() {
        use crate::necklace_perm;
        let result = necklace_perm(vec![1, 2, 3, 4]);
        println!("{:?}", result);
        assert_eq!(result.len(), 3);
        assert!(result.contains(&vec![1, 2, 4, 3]));
        assert!(result.contains(&vec![1, 2, 3, 4]));
        assert!(result.contains(&vec![1, 3, 2, 4]));
    }

    #[test]
    fn test_perm2_order_5() {
        use crate::necklace_perm;
        let result = necklace_perm(vec![1, 2, 3, 4, 5]);
        println!("{:?}", result);
        assert_eq!(result.len(), 12);
        assert!(result.contains(&vec![1, 2, 4, 5, 3]));
        assert!(result.contains(&vec![1, 2, 5, 4, 3]));
        assert!(result.contains(&vec![1, 2, 3, 5, 4]));
        assert!(result.contains(&vec![1, 2, 5, 3, 4]));
        assert!(result.contains(&vec![1, 2, 3, 4, 5]));
        assert!(result.contains(&vec![1, 2, 4, 3, 5]));
        assert!(result.contains(&vec![1, 3, 2, 5, 4]));
        assert!(result.contains(&vec![1, 3, 5, 2, 4]));
        assert!(result.contains(&vec![1, 3, 2, 4, 5]));
        assert!(result.contains(&vec![1, 3, 4, 2, 5]));
        assert!(result.contains(&vec![1, 4, 2, 3, 5]));
        assert!(result.contains(&vec![1, 4, 3, 2, 5]));
    }

    #[test]
    fn test_perm2_order_6() {
        use crate::necklace_perm;
        let result = necklace_perm(vec![1, 2, 3, 4, 5, 6]);
        println!("{:?}", result);
        assert_eq!(result.len(), 60);
    }

    #[test]
    fn test_perm2_order_7() {
        use crate::necklace_perm;
        let result = necklace_perm(vec![1, 2, 3, 4, 5, 6, 7]);
        println!("{:?}", result);
        assert_eq!(result.len(), 360);
    }

    #[test]
    fn test_perm2_order_8() {
        use crate::necklace_perm;
        let result = necklace_perm(vec![1, 2, 3, 4, 5, 6, 7, 8]);
        println!("{:?}", result);
        assert_eq!(result.len(), 2520);
    }}