use rand::distr::Distribution;

fn r1(rng: &mut rand::rngs::ThreadRng, mean: f32, std_dev: f32) -> f32 {
    let normal_dist = rand_distr::Normal::new(mean, std_dev).unwrap();
    normal_dist.sample(rng)
}

fn r_generic<F>(rng: &mut rand::rngs::ThreadRng, mean: F, std_dev: F) -> F
where
    F: num_traits::Float,
    rand_distr::StandardNormal: rand_distr::Distribution<F>,
{
    let normal_dist = rand_distr::Normal::new(mean, std_dev).unwrap();
    normal_dist.sample(rng)
}

fn main() {
    let mut rng = rand::rng();
    let (mean, std_dev): (f32, f32) = (0., 0.6);
    let normal_dist = rand_distr::Normal::new(mean, std_dev).unwrap();
    let v = normal_dist.sample(&mut rng);
    println!("{v:.2} is from a N({mean:.2}, {std_dev:.2}) distribution");

    // Test r1 function
    let v = r1(&mut rng, mean, std_dev);
    println!("{v:.2} is from a N({mean:.2}, {std_dev:.2}) distribution");

    // Test ThreadRng::default()
    let mut rng = rand::rngs::ThreadRng::default();
    let (mean, std_dev): (f32, f32) = (0., 0.6);
    let normal_dist = rand_distr::Normal::new(mean, std_dev).unwrap();
    let v = normal_dist.sample(&mut rng);
    println!("{v:.2} is from a N({mean:.2}, {std_dev:.2}) distribution");

    // Test r_generic function
    let v = r_generic(&mut rng, mean, std_dev);
    println!("{v:.2} is from a N({mean:.2}, {std_dev:.2}) distribution");
}
