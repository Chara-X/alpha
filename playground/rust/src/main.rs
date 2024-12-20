static X: SomeStruct = SomeStruct { x: 1 };
struct SomeStruct {
    x: i32,
}
const FOO: SomeStruct = SomeStruct { x: 1 };
const BAR: SomeStruct = FOO;
fn main() {
    // let a = X;
    // let b = X;
}
