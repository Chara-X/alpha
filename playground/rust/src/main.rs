fn main() {
    let p = PeopleImpl {};
    do_something(p);
}
fn do_something(u: impl serde::de::DeserializeOwned) {}
trait People {
    fn walk(&self);
}
trait User: People {}
struct PeopleImpl {}
impl People for PeopleImpl {
    fn walk(&self) {}
}
impl User for PeopleImpl {}
impl<'de> serde::Deserialize<'de> for PeopleImpl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        todo!()
    }
}
