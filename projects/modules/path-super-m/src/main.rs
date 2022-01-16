use path_super_m::foo::v1::depth;
use path_super_m::foo::v2::index;

fn main() {
    assert_eq!(depth(), 2);
    assert_eq!(index(), 1);
}
