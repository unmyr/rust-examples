pub mod foo {
    pub mod v1 {
        pub mod bar {
            use std::ops::Add;

            pub fn f1() {}
            pub fn f2() {}
            pub fn add<T: Add<Output=T>>(a: T, b: T) -> T {
                a.add(b)
            }
        }
        mod tests {
            #[test]
            fn test_v1() {
                super::bar::f1();
                super::bar::f2();
                assert_eq!(
                    super::bar::add(1, 2), 3
                );
            }
        }
    }
    pub mod v2 {
        pub mod bar {
            use std::ops::Add;

            pub fn f1() {}
            pub fn f2() {}
            pub fn add<T: Add<Output=T>>(a: T, b: T) -> T {
                a.add(b)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_v2() {
        use super::foo::v2::bar;
        use super::foo::v2::bar::{f1, f2};
        f1();
        f2();
        assert_eq!(bar::add(1, 2), 3);
    }
}
