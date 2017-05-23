extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;

use enum_index::{EnumIndex, IndexEnum};

#[allow(dead_code)]
#[derive(EnumIndex)]
enum TestEnumFirst {
    VariantA,
    VariantB(bool),
    VariantC{x: f32, y: u64}
}

#[test]
fn test_enum_index_derive() {
    assert_eq!(TestEnumFirst::VariantA.enum_index(), 0usize);
    assert_eq!(TestEnumFirst::VariantB(false).enum_index(), 1usize);
    assert_eq!(TestEnumFirst::VariantC { x: 0f32, y: 0u64}.enum_index(), 2usize);
}


#[allow(dead_code)]
#[derive(IndexEnum, PartialEq, Debug)]
enum TestEnumSecond {
    VariantA,
    VariantB,
    VariantC
}

#[test]
fn test_index_enum_derive() {
    assert_eq!(TestEnumSecond::index_enum(0), Some(TestEnumSecond::VariantA));
    assert_eq!(TestEnumSecond::index_enum(1), Some(TestEnumSecond::VariantB));
    assert_eq!(TestEnumSecond::index_enum(2), Some(TestEnumSecond::VariantC));
    assert_eq!(TestEnumSecond::index_enum(3), None);
}