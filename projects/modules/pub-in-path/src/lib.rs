// use self::pub_mod::inner_two;  // NG: function `inner_two` is private
pub mod pub_mod {
    pub(in crate::pub_mod) fn inner_two() -> u8 { 2 }
    pub fn two() -> u8 { inner_two() }

    pub mod sub_mod {
        pub fn two() -> u8 { super::inner_two() }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(super::inner_two(), 2); // OK
            assert_eq!(super::two(), 2);
        }
    }    
}

pub mod other_mod {
    // pub fn two() -> u8 { super::pub_mod::inner_two() } // NG
    pub fn two() -> u8 { super::pub_mod::two() } // OK
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // assert_eq!(super::pub_mod::inner_two(), 2); // NG
        assert_eq!(super::pub_mod::two(), 2);
        assert_eq!(super::other_mod::two(), 2);
    }
}
