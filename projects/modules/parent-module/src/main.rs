use parent_module::foo::v1::depth;
use parent_module::foo::v2::index;

fn main() {
    assert_eq!(depth(), 2);
    assert_eq!(index(), 1);
}
