use super::depth as parent_depth;
use super::v1::index as prev_index;

pub fn depth() -> u8 {
    parent_depth() + 1
}

pub fn index() -> u8 { prev_index() + 1 }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::depth(), 2);
        assert_eq!(super::index(), 1);
    }
}