use num_traits::Float;

fn main() {
    let arr2 = ndarray::Array2::from(vec![[1., 2., 3.], [1., 2., 3.11111]]);
    let arr2_row0 = &arr2.row(0);
    let arr2_row1 = &arr2.row(1);
    println!("{:.3?}", arr2_row0 * arr2_row1);
    println!("{:.3?}", &arr2.row(0) * &arr2.row(1));
    let w_str = &arr2
        .rows()
        .into_iter()
        .map(|row| format!("{:.2?}", row.to_vec()))
        .collect::<Vec<_>>()
        .join(", ");
    println!("arr2=[{}]", w_str);
    println!(
        "Cosine similarity={:.4?}",
        (&arr2.row(0) * &arr2.row(1)).sum()
            / (&arr2.row(0).mapv(|v| v * v).sum().powf(0.5)
                * &arr2.row(1).mapv(|v| v * v).sum().powf(0.5))
    );

    println!("----");
    let arr2 = ndarray::Array2::from(vec![[1., 0., 0.], [0., 1., 0.]]);
    let arr2_row0 = &arr2.row(0);
    let arr2_row1 = &arr2.row(1);
    println!(
        "arr2={}",
        format!("{:.1?}", arr2).lines().collect::<Vec<_>>().join("")
    );
    println!("arr2={}", format!("{:.1?}", arr2).replace("\n", ""));
    println!("{:?}", arr2_row0 * arr2_row1);
    println!("{:?}", &arr2.row(0) * &arr2.row(1));
    println!(
        "Cosine similarity={:.4?}",
        (&arr2.row(0) * &arr2.row(1)).sum()
            / (&arr2.row(0).mapv(|v| v * v).sum().powf(0.5)
                * &arr2.row(1).mapv(|v| v * v).sum().powf(0.5))
    );
}
