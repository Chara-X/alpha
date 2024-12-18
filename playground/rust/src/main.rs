use std::fmt::{Debug, Display};

#[derive(core::clone::Clone, core::default::Default, Debug)]
struct SomeOptions {
    foo: i32,
    bar: f32,
    baz: String,
}
trait SomeTrait {
    fn some_method(&self) {}
}
fn compare<T: Eq>(x: T, y: T) -> bool {
    x == y
}
fn main() {}
