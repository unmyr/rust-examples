fn depth() -> u8 { 0 }
pub mod foo {
    fn depth() -> u8 { super::depth() + 1 }
    pub mod v1 {
        pub fn depth() -> u8 {
            super::depth() + 1
        }
        pub fn index() -> u8 { 0 }

        #[cfg(test)]
        mod tests;
    }
    pub mod v2;
}
