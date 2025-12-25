use largest::{largest_char_b, largest_i32_b};
use largest::{largest_char_m, largest_i32_m};

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32_b(&number_list);
    println!("largest_i32_b:: The largest number is {}", result);

    let result = largest_i32_m(&number_list);
    println!("largest_i32_m:: The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char_b(&char_list);
    println!("largest_char_b:: The largest char is {}", result);

    let result = largest_char_m(&char_list);
    println!("largest_char_m:: The largest char is {}", result);
}
