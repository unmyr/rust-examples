pub mod foo {
    pub fn sum() -> u8 { self::a::index() + self::b::index() }
    pub mod a {
        pub fn index() -> u8 { 1 }
    }
    pub mod b {
        pub fn index() -> u8 { 2 }
    }
}

#[cfg(test)]
mod tests {
    use super::foo;

    #[test]
    fn it_works() {
        assert_eq!(foo::sum(), 3);
        assert_eq!(foo::a::index(), 1); 
        assert_eq!(foo::b::index(), 2); 
    }
}