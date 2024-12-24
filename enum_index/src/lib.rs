#![cfg_attr(not(any(feature = "std", test)), no_std)]

pub trait EnumIndex {
    fn enum_index(&self) -> usize;
}

// Trait for converting an index to the corresponding enum, only works for C-Like enums for now
pub trait IndexEnum {
    fn index_enum(index: usize) -> Option<Self> where Self: Sized;
}
