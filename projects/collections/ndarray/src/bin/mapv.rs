use ndarray::Array1;

fn main() {
    // Create a 1D array
    let arr = Array1::from(vec![1, 2, 3, 4, 5]);

    // Square each element using mapv
    let squared = arr.mapv(|x| x * x);

    println!("Original: {:?}", arr);
    println!("Squared:  {:?}", squared);
}
