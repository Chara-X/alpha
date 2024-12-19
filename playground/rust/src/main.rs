use std::any::Any;

trait MyTrait {
    fn do_something(&self);
}

struct MyStruct<'a> {
    value: i32,
    text: &'a str,
}

impl<'a> MyTrait for MyStruct<'a> {
    fn do_something(&self) {
        self.do_something();
        println!("Value: {}", self.value);
    }
}
fn main() {
    let s = &MyStruct {
        value: 42,
        text: "",
    };
    let t = s as &dyn Any;
    let t = t.downcast_ref::<String>().unwrap();
    let s: &'static str = "I have a static lifetime.";
}
