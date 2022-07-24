use collatz::collatz_m;
 
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let num = &args[1];
    println!("{:?}", collatz_m(num.parse().unwrap()));
}
 
