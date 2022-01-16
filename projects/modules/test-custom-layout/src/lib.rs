/// # Examples
///
/// ```
/// use test_custom_layout::add;
/// assert_eq!(add(1, 2), 3);
/// ```
pub fn add(x: u8, y: u8) -> u8 { x + y }

#[cfg(test)]
mod tests_custom_loc;
