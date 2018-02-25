/// Constructs a new `Rc<T>`
/// # Examples
///
/// ```
/// use std::rc::Rc;
///
/// let five = Rc::new(5)
/// ```
#[cfg(any(windows))]
pub fn add_two(a: i32) -> i32 {
    a + 2
}