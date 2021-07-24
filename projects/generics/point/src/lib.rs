// Tuple structs are similar to regular structs,
// but its fields have no names.
#[allow(dead_code)]
struct TupleStaticPair(u32, String);

#[allow(dead_code)]
struct TupleGenericPairSameType<T> (T, T);

impl<T> TupleGenericPairSameType<T> {
    #[allow(dead_code)]
    fn first(&self) -> &T {
        &self.0
    }

    #[allow(dead_code)]
    fn second(&self) -> &T {
        &self.1
    }
}

// Structs with named fields
#[allow(dead_code)]
struct PointSameType<T> {
    x: T,
    y: T,
}

impl<T> PointSameType<T> {
    #[allow(dead_code)]
    fn x(&self) -> &T {
        &self.x
    }
}

#[allow(dead_code)]
struct PointDifferentType<U, V> {
    x: U,
    y: V,
}

impl<U, V> PointDifferentType<U, V> {
    #[allow(dead_code)]
    fn x(&self) -> &U {
        &self.x
    }

    #[allow(dead_code)]
    fn y(&self) -> &V {
        &self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_static() {
        let pair_u32_string = TupleStaticPair(5, String::from("Hello"));
        assert_eq!(pair_u32_string.0, 5);
        assert_eq!(pair_u32_string.1, "Hello");
    }

    #[test]
    fn test_tuple_generic() {
        let p_both_int = TupleGenericPairSameType(5, 3);
        assert_eq!(p_both_int.0, 5);
        assert_eq!(p_both_int.1, 3);
        assert_eq!(p_both_int.first(), &5);
        assert_eq!(p_both_int.second(), &3);
    }

    #[test]
    fn test_named_same_type() {
        let p_both_int = PointSameType { x: 5, y: 3 };
        assert_eq!(p_both_int.x, 5);
        assert_eq!(p_both_int.y, 3);
        assert_eq!(p_both_int.x(), &5);

        let p_both_str = PointSameType { x: "Hello", y: "World" };
        assert_eq!(p_both_str.x, "Hello");
        assert_eq!(p_both_str.y, "World");
        assert_eq!(p_both_str.x(), &"Hello");
    }

    #[test]
    fn test_named_different_types() {
        let p_both_int = PointDifferentType { x: 5, y: 3.14 };
        assert_eq!(p_both_int.x, 5);
        assert_eq!(p_both_int.y, 3.14);

        let pair_str_char = PointDifferentType { x: "Hello", y: 'c' };
        assert_eq!(pair_str_char.x, "Hello");
        assert_eq!(pair_str_char.y, 'c');
        assert_eq!(pair_str_char.x(), &"Hello");
        assert_eq!(pair_str_char.y(), &'c');
    }
}

