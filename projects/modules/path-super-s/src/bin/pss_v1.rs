use path_super_s::foo::v1;

fn main() {
    assert_eq!(v1::depth(), 2);
    assert_eq!(v1::index(), 0);
}
