use std::cell::Cell; 

pub struct Point {
    pub x: Cell<i32>,
    pub y: i32
}

impl Point {
    /// # Examples
    /// 
    /// ```
    /// use interior_mutability::Point;
    /// 
    /// let point = Point { x: std::cell::Cell::new(1), y: 0 };
    /// point.increment();
    /// assert_eq!(point.x.get(), 2);
    /// ```
    pub fn increment(&self) { // note: no mut again
        self.x.set(self.x.get() + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_direct() {
        let point = Point {
            x: Cell::new(1), // interior mutability using Cell
            y: 0
        };
        point.x.set(2);
        assert_eq!(point.x.get(), 2);
    }

    #[test]
    fn test_im_method() {
        let point = Point {
            x: Cell::new(1), // interior mutability using Cell
            y: 0
        };
        point.increment();
        assert_eq!(point.x.get(), 2);
    }
}
