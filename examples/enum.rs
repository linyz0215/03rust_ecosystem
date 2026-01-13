use anyhow::Result;
use strum::IntoEnumIterator;
use strum::Display;
use strum::{EnumCount, EnumDiscriminants,EnumIs,EnumIter,EnumString,IntoStaticStr, VariantNames};
#[derive(Debug, EnumString, EnumCount, EnumDiscriminants, EnumIter, EnumIs, IntoStaticStr, VariantNames)]

#[allow(unused)]
enum MyEnum {
    A,
    B(String),
    C,
}

#[allow(unused)]
#[derive(Display, Debug)]
enum Color {
    #[strum(serialize = "red_color")]
    Red,
    Greem {
        range: usize,
    },
    Blue(usize),
    Yellow,
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize,
    }
}

fn main() -> Result<()> {
    println!("{:?}", MyEnum::VARIANTS);
    MyEnum::iter().for_each(|v| println!("{:?}", v));

    let my_enum = MyEnum::B("hello".to_string());
    println!("{:?}",my_enum.is_b());
    let s: &'static str = my_enum.into();
    println!("{}", s);
    println!("{:?}",MyEnum::COUNT);
    let color =  Color::Red;
    println!("{}", color);
    Ok(())
}