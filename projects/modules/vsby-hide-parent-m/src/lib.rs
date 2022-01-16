pub use inner_mod::one;

mod inner_mod { // private
    fn inner_one() -> u8 { 1 }
    pub fn one() -> u8 { inner_one() }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(super::inner_one(), 1);
            assert_eq!(super::one(), 1);
        }
    }
}
