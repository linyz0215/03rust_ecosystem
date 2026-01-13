
use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, Display,Clone,Copy,From, Add, Into)]
struct MyInt(i32);

#[derive(PartialEq, From, Add, Into)]
struct Point2D {
    x: i64,
    y: i64,
}

#[derive(PartialEq, From, Add, Display)]
enum MyEnum {
    #[display("int: {_0}")]
    Int(i32),
    Uint(u32),
    #[display("nothing")]
    Nothing,
}


fn main() {
    let myint:MyInt = 10.into();
    let v = myint + 20.into();
    let v1: i32 = v.into();
    println!("{} {} {}", myint, v, v1);
    let e: MyEnum = 10i32.into();
    let e2: MyEnum = 20u32.into();
    let e3 = MyEnum::Nothing;
    println!("{} {} {}", e, e2, e3);
}