/// Generates an N-dimensional rotation matrix (rotates in the plane of the i and j axes)
fn rotation_matrix<F: num_traits::Float>(
    n: usize,
    i: usize,
    j: usize,
    theta: F,
) -> ndarray::Array2<F> {
    assert!(i < n && j < n && i != j, "Invalid rotation axes");
    let mut rot = ndarray::Array2::<F>::eye(n);
    let (cos_t, sin_t) = (theta.cos(), theta.sin());
    (rot[[i, i]], rot[[i, j]]) = (cos_t, -sin_t);
    (rot[[j, i]], rot[[j, j]]) = (sin_t, cos_t);
    rot
}

fn main() {
    // Example: Rotate a 2D vector 90 degrees in the (x, y) plane
    let v1 = ndarray::arr2::<f32, 2>(&[[1., 0.]]).reversed_axes();
    let rot = rotation_matrix::<f32>(v1.dim().0, 0, 1, std::f32::consts::FRAC_PI_2);
    let v_rot = rot.dot(&v1);
    println!("v_rot={:.1?}", v_rot);
    let error = (&v_rot - ndarray::arr2(&[[0.], [1.]])).powf(2.).sum();
    println!("error={:?}", error);
    assert!(error < 1e-14); // f32
    // assert!(error < 1e-32); // f64

    // Example: Rotate a 3D vector 90 degrees in the (x, y) plane
    let v1 = ndarray::arr2::<f32, 3>(&[[1., 0., 0.]]).reversed_axes();
    let rot = rotation_matrix(v1.dim().0, 0, 1, std::f32::consts::FRAC_PI_2);
    let v_rot = rot.dot(&v1);
    println!("v_rot={:.1?}", v_rot);
    let error = (&v_rot - ndarray::arr2(&[[0.], [1.], [0.]])).powf(2.).sum();
    println!("error={:?}", error);
    assert!(error < 1e-14);
}
