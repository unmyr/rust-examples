/// # Examples
///
/// ```
/// use separating_modules::v2::version;
/// assert_eq!(version(), 2);
/// ```
pub fn version() -> u8 { 2 }

#[cfg(test)]
mod tests;
