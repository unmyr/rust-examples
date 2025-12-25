pub fn largest_i32_m(list: &[i32]) -> i32 {
    let mut largest = list[0]; // copy

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_i32_b(list: &[i32]) -> &i32 {
    let mut largest = &list[0]; // borrowing

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_char_m(list: &[char]) -> char {
    let mut largest = list[0]; // copy

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_char_b(list: &[char]) -> &char {
    let mut largest = &list[0]; // borrowing

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_gen_m<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; // copy

    for &item in list {
        if item > largest {
            largest = item; // copy
        }
    }

    largest
}

pub fn largest_gen_b<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0]; // borrowing

    for item in list {
        if item > largest {
            largest = item; // borrowing
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_largest_i32_b() {
        use crate::largest_i32_b;

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_i32_b(&number_list);
        assert_eq!(*result, 100);
    }

    #[test]
    fn test_largest_i32_m() {
        use crate::largest_i32_m;

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_i32_m(&number_list);
        assert_eq!(result, 100);
    }

    #[test]
    fn test_largest_char_b() {
        use crate::largest_char_b;

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest_char_b(&char_list);
        assert_eq!(*result, 'y');
    }

    #[test]
    fn test_largest_char_m() {
        use crate::largest_char_m;

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest_char_m(&char_list);
        assert_eq!(result, 'y');
    }

    #[test]
    fn test_largest_gen_m() {
        use crate::largest_gen_m;

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_gen_m(&number_list);
        assert_eq!(result, 100);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest_gen_m(&char_list);
        assert_eq!(result, 'y');
    }

    #[test]
    fn test_largest_gen_b() {
        use crate::largest_gen_b;

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_gen_b(&number_list);
        assert_eq!(*result, 100);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest_gen_b(&char_list);
        assert_eq!(*result, 'y');
    }
}
