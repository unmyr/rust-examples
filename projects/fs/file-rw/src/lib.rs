use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<(usize, String), String> {
    let mut f = match File::open(&path) {
        Ok(file) => file,
        Err(error) => {
            return Err(format!("Problem opening the file: path={:?} error={:?}", path.as_ref(), error))
        },
    };

    let mut contents = String::new();
    let read_size = match f.read_to_string(&mut contents) {
        Ok(read_size) => read_size,
        Err(error) => {
            return Err(format!("Problem reading the file: path={:?} error={:?}", path.as_ref(), error))
        },
    };
    Ok((read_size, contents))
}

pub fn write_to_file<P: AsRef<Path>>(path: P, data: &[u8]) -> usize{
    let mut f = match OpenOptions::new().write(true).truncate(true).create(true).open(&path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let write_size = match f.write(data) {
        Ok(write_size) => write_size,
        Err(error) => panic!("Problem writing the file: {:?}", error),
    };
    write_size
}

#[cfg(test)]
mod tests {
    use crate::{write_to_file, read_from_file};

    #[test]
    fn test_rw_path_str() {
        let path = "test_rw_path_str.txt";

        let write_size = write_to_file(path, b"hello 12345");
        assert_eq!(write_size, 11);
        let (read_size, contents) = read_from_file(path).unwrap();
        assert_eq!(read_size, 11);
        assert_eq!(contents, "hello 12345");

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_rw_path() {
        let path = std::path::Path::new("test_rw_str_path.txt");

        let write_size = write_to_file(path, b"hello");
        assert_eq!(write_size, 5);
        let (read_size, contents) = read_from_file(path).unwrap();
        assert_eq!(read_size, 5);
        assert_eq!(contents, "hello");

        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_error_read_no_file() {
        let path = std::path::Path::new("404.txt");
        assert_eq!(
            read_from_file(path),
            Err(String::from("Problem opening the file: path=\"404.txt\" error=Os { code: 2, kind: NotFound, message: \"No such file or directory\" }"))
        )
     }

     #[test]
     fn test_write_string() {
        let path = std::path::Path::new("test_write_string.txt");

        let s = String::from("こんにちは世界");
        let write_size = write_to_file(path, s.as_bytes());
        assert_eq!(write_size, 21);
        let (read_size, contents) = read_from_file(path).unwrap();
        assert_eq!(read_size, 21);
        assert_eq!(contents, "こんにちは世界");

        std::fs::remove_file(path).unwrap();
      }
}
