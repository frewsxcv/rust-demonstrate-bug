extern crate meow;

use meow::{Foo, MyEnum};


#[test]
fn test() {
    MyEnum::Foo(Foo(5));
}
