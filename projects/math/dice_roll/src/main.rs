use rand::prelude::*;

fn main() {
    let mut rng = rand::rng();

    // dice roll: generate 10 random integers between 1 and 6
    let numbers: Vec<u8> = (0..10).map(|_| rng.random_range(1..=6)).collect();
    println!("Dice rolls: {:?}", numbers);

    // Shuffle dice faces
    let mut dice_faces: Vec<u8> = (1..=6).collect();
    dice_faces.shuffle(&mut rng);
    println!("Shuffled dice faces: {:?}", dice_faces);
}
