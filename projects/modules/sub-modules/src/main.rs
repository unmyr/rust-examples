use sub_modules::foo::v1::bar;
use sub_modules::foo::v2::bar as bar_v2;

fn main() {
    bar::f1();
    bar::f2();
    assert_eq!(bar::add(1, 2), 3);

    bar_v2::f1();
    bar_v2::f2();
    assert_eq!(bar_v2::add(1, 2), 3);
}
