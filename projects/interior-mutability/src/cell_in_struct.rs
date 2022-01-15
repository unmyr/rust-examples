use std::cell::Cell;

pub struct Point {
    pub x: u8,
    pub y: Option<Cell<u8>>,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Point { x: x, y: Some(Cell::new(y)) }
    }

    /// # Examples
    ///
    /// ```
    /// use interior_mutability::cell_in_struct::Point;
    ///
    /// let mut p: Point = Point::new(0, 1);
    /// p.increment_both();
    /// assert_eq!(p.x, 1);
    /// assert_eq!(p.y.unwrap().get(), 2);
    /// ```
    pub fn increment_both(&mut self) {  // require mutable instance
        self.x += 1;
        if let Some(ref y) = self.y {
            y.set(y.get() + 1);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use interior_mutability::cell_in_struct::Point;
    ///
    /// let p: Point = Point::new(0, 1);
    /// p.increment_y();
    /// assert_eq!(p.y.unwrap().get(), 2);
    /// ```
    pub fn increment_y(&self) {
        // self.x += 1;
        if let Some(ref y) = self.y {
            y.set(y.get() + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_im_direct() {
        let p: Point = Point::new(0, 1);
        if let Some(ref y) = p.y {
            y.set(2);
        }
        assert_eq!(p.y.map(|y| y.get()), Some(2));
    }

    #[test]
    fn test_increment_both() {
        let mut p: Point = Point::new(0, 1);
        // let x_ref = &mut p.x;

        p.increment_both();
        // *x_ref += 1;

        assert_eq!(p.x, 1);
        assert_eq!(p.y.map(|y| y.get()), Some(2));
    }

    #[test]
    fn test_increment_y() {
        let p: Point = Point::new(0, 1);
        let p_ref = &p;

        p.increment_y();
        p_ref.increment_y();

        assert_eq!(p.y.map(|y| y.get()), Some(3));
    }
}
