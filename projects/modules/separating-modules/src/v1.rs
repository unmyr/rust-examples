/// # Examples
///
/// ```
/// use separating_modules::v1::version;
/// assert_eq!(version(), 1);
/// ```
pub fn version() -> u8 { 1 }

#[cfg(test)]
mod tests;
