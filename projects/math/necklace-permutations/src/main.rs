use necklace_permutations::necklace_perm;

fn main() {
    let result = necklace_perm(vec![1, 2, 3, 4, 5]);
    println!("{:?}", result.len());
    println!("{:?}", result);
}