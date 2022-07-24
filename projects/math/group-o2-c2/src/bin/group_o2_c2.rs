use std::collections::VecDeque;
use group_o2_c2::{xor_inv, xor_calc, Xor};

fn main() {
    println!("{}^(-1) = {}", Xor::G, xor_inv(&Xor::G));
    println!("{}^(-1) = {}", Xor::E, xor_inv(&Xor::E));

    let v1_in = [Xor::G, Xor::G, Xor::G];
    let v1_str = v1_in.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ⊕ ");
    let v1_out = xor_calc(VecDeque::from(v1_in));
    println!(
        "{} = {}", v1_str,
        v1_out.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
    );

    let v2_in = [Xor::G, Xor::G, Xor::G, Xor::G];
    let v2_str = v2_in.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ⊕ ");
    let v2_out = xor_calc(VecDeque::from(v2_in));
    println!(
        "{} = {}", v2_str,
        v2_out.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
    );
}
