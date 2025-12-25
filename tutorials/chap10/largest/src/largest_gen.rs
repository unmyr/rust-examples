use largest::largest_gen_b;
use largest::largest_gen_m;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_gen_b(&number_list);
    println!("largest_gen_b:: The largest number is {}", result);

    let result = largest_gen_m(&number_list);
    println!("largest_gen_m:: The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_gen_b(&char_list);
    println!("largest_gen_b:: The largest char is {}", result);

    let result = largest_gen_m(&char_list);
    println!("largest_gen_m:: The largest char is {}", result);
}
