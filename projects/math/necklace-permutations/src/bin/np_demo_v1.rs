use necklace_permutations::necklace_perm_with_filter;

fn main() {
    let cmp_np = |a: &Vec<u8>, b: &Vec<u8>| {
        let mut compare_order = (0..(a.len())).collect::<Vec<usize>>();
        let end = a.len() - 1;
        if end > 2 {
            for i in 2..end {
                compare_order.swap(i, end);
            }
        }
        for i in compare_order {
            if a[i] != b[i] {
                return a[i].partial_cmp(&b[i]).unwrap();
            }
        }
        return a[end].partial_cmp(&b[end]).unwrap();
    };

    let mut result = necklace_perm_with_filter(vec![1, 2, 3, 4]);
    result.sort_by(cmp_np);
    println!("{:?}", result.len());
    for v in result {
        println!("{:?}", v);
    }

    let mut result = necklace_perm_with_filter(vec![1, 2, 3, 4, 5]);
    println!("{:?}", result.len());
    result.sort_by(cmp_np);
    for v in result {
        println!("{:?}", v);
    }
}