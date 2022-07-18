use primes::primes;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let from = &args[1];
    let to = &args[2];
    let result =  primes(from.parse().unwrap(), to.parse().unwrap());
    println!("{:?} {:?}", result.len(), result);
}
