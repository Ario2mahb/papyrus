use std::fmt::Debug;

use starknet_api::core::ContractAddress;
use starknet_api::hash::StarkHash;
use starknet_api::state::StorageKey;

use crate::db::serialization::StorageSerde;
use crate::test_utils::GetTestInstance;

pub trait StorageSerdeTest: StorageSerde {
    fn storage_serde_test();
}

// Implements the [`storage_serde_test`] function for every type that
// implements the [`StorageSerde`] and [`GetTestInstance`] traits.
impl<T: StorageSerde + GetTestInstance + Eq + Debug> StorageSerdeTest for T {
    fn storage_serde_test() {
        let item = T::get_test_instance();
        let mut serialized: Vec<u8> = Vec::new();
        item.serialize_into(&mut serialized).unwrap();
        let bytes = serialized.into_boxed_slice();
        let deserialized = T::deserialize_from(&mut bytes.as_ref());
        assert_eq!(item, deserialized.unwrap());
    }
}

// Tests all types that implement the [`StorageSerde`] trait
// via the [`auto_storage_serde`] macro.
macro_rules! create_storage_serde_test {
    ($name:ident) => {
        paste::paste! {
            #[test]
            fn [<"storage_serde_test" _$name:snake>]() {
                $name::storage_serde_test()
            }
        }
    };
    (($ty0:ty, $ty1:ty)) => {
        paste::paste! {
            #[test]
            fn [<"storage_serde_test" _$ty0:snake _$ty1:snake>]() {
                <($ty0, $ty1)>::storage_serde_test()
            }
        }
    };
    (($ty0:ty, $ty1:ty, $ty2:ty)) => {
        paste::paste! {
            #[test]
            fn [<"storage_serde_test" _$ty0:snake _$ty1:snake _$ty2:snake>]() {
                <($ty0, $ty1, $ty2)>::storage_serde_test()
            }
        }
    };
}
pub(crate) use create_storage_serde_test;

////////////////////////////////////////////////////////////////////////
// Implements the [`GetTestInstance`] trait for types not supported
// by the macro [`impl_get_test_instance`] and calls the [`create_test`]
// macro to create the tests for them.
////////////////////////////////////////////////////////////////////////
create_storage_serde_test!(StarkHash);
create_storage_serde_test!(ContractAddress);
create_storage_serde_test!(StorageKey);