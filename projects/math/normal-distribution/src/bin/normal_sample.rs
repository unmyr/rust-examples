use rand::distr::Distribution;

fn main() {
    let mut rng = rand::rng();
    let (mean, std_dev): (f32, f32) = (0., 0.6);
    let normal_dist = rand_distr::Normal::new(mean, std_dev).unwrap();
    let v = normal_dist.sample(&mut rng);
    println!("{v:.2} is from a N({mean:.2}, {std_dev:.2}) distribution");
}
