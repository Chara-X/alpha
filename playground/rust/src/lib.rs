pub use cal::add;
pub mod cal;
pub fn hello() {
    add(1, 2);
}
/// # Examples:
/// ```
/// use rust::test;
/// let v = vec![1, 2, 3];
/// assert_eq!(test(v.into_iter()), 3);
/// ```
pub fn test<T: Iterator>(it: T) -> usize {
    it.count()
}
