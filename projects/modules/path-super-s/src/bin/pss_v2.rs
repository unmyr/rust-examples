use path_super_s::foo::v2::{depth, index};

fn main() {
    assert_eq!(depth(), 2);
    assert_eq!(index(), 1);
}
