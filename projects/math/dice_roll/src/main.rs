use rand::prelude::SliceRandom;
use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    // println!("Float: {}", rng.random_range(1.0..=6.0));

    // dice roll: generate 10 random integers between 1 and 6
    let numbers: Vec<u32> = (0..10).map(|_| rng.random_range(1..=6)).collect();
    println!("Dice rolls: {:?}", numbers);

    // Shuffle dice faces
    let mut dice_faces: Vec<i32> = (1..=6).collect();
    dice_faces.shuffle(&mut rng);
    println!("Shuffled dice faces: {:?}", dice_faces);
}
