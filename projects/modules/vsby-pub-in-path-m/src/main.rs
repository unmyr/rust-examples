// use pub_in_path::pub_mod::inner_two; // NG: function `inner_two` is private
use pub_in_path::pub_mod::two;
use pub_in_path::pub_mod::sub_mod::two as sub_two;

fn main() {
    assert_eq!(two(), 2);
    assert_eq!(sub_two(), 2);
}
