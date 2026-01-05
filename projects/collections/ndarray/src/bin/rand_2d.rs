use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    // Initialize the matrix with random values ​​in the range [-0.5,0.5]
    let h1 = ndarray::Array2::<f32>::from_shape_fn((1, 2), |_| rng.random_range(-0.5..0.5));
    println!("h1={:?}", h1);

    // Initialize the matrix with 0 and then overwrite it with random numbers in the range [-0.5, 0.5].
    let mut h2 = ndarray::Array2::<f32>::zeros(h1.dim());
    h2.map_inplace(|v| *v = rng.random_range(-0.5..0.5));
    println!("h2={:?}", &h2);

    let h3 = ndarray::Array2::<f32>::from_elem(h1.dim(), rng.random_range(-0.5..0.5));
    println!("h3={:?}", &h3);

    // Add 3 rows of random numbers to a 0-by-2 matrix.
    let rows: usize = 3;
    let mut h4 = ndarray::Array2::<f32>::zeros((0, 2));
    assert!(h4.shape()[0] == 0);
    assert!(h4.shape()[1] == 2);
    (0..rows).for_each(|_| {
        h4.push_row(ndarray::ArrayView::<f32, ndarray::Ix1>::from(
            ndarray::Array1::<f32>::from_shape_fn(h4.shape()[1], |_| rng.random_range(-0.5..0.5))
                .view(),
        ))
        .ok();
    });
    println!("h4={:?}", h4);
    assert!(h4.shape()[0] == rows);
    assert!(h4.shape()[1] == 2);

    // Add two random columns to a 3-by-0 matrix.
    let columns: usize = 2;
    let mut h5 = ndarray::Array2::<f32>::zeros((3, 0));
    assert!(h5.shape()[0] == 3);
    assert!(h5.shape()[1] == 0);
    (0..columns).for_each(|_| {
        h5.push_column(ndarray::ArrayView::<f32, ndarray::Ix1>::from(
            ndarray::Array1::<f32>::from_shape_fn(h5.shape()[0], |_| rng.random_range(-0.5..0.5))
                .view(),
        ))
        .ok();
    });
    println!("h5={:?}", h5);
    assert!(h5.shape()[0] == 3);
    assert!(h5.shape()[1] == columns);
}
