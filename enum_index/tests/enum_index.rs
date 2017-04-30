extern crate enum_index;
#[macro_use]
extern crate enum_index_derive;

use enum_index::EnumIndex;

#[allow(dead_code)]
#[derive(EnumIndex)]
enum TestEnum {
    VariantA,
    VariantB(bool),
    VariantC{x: f32, y: u64}
}

#[test]
fn test_derive() {
    assert_eq!(TestEnum::VariantA.get_index(), 0usize);
    assert_eq!(TestEnum::VariantB(false).get_index(), 1usize);
    assert_eq!(TestEnum::VariantC { x: 0f32, y: 0u64}.get_index(), 2usize);
}