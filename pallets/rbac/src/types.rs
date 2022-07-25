use super::*;
use frame_support::pallet_prelude::*;


pub struct Permission {
    name: BoundedVec<u8, ConstU32<50>>,
    value: bool,
}