use crate::State;
use crate::web::models::context::Context;
use crate::web::models::user_model::User;

#[macro_export]
macro_rules! map_get {
    ($map:expr,$key: expr) => {
        $map.with(|map| map.borrow_mut().get($key))
    };
}
#[macro_export]
macro_rules! map_insert {
    ($map: expr,$key: expr,$data: expr) => {
        $map.with(|map| map.borrow_mut().insert($key, $data))
    };
}
macro_rules! map_remove {
    ($map: expr,$key: expr) => {
        $map.with(|map| map.borrow_mut().remove($key))
    };
}

#[macro_export]
macro_rules! impl_storable {
    ($type:ident <$gen:ident >) => {
        use ic_stable_structures::storable::Bound;
        use ic_stable_structures::Storable;
        use std::borrow::Cow;

        impl Storable for $type<$gen> {
            fn to_bytes(&self) -> Cow<[u8]> {
                let bytes = bincode::serialize(self).expect("Serialization failed");
                Cow::Owned(bytes)
            }

            fn from_bytes(bytes: Cow<[u8]>) -> Self {
                bincode::deserialize(&bytes).expect("Deserialization failed")
            }

            const BOUND: Bound = Bound::Bounded {
                max_size: 10_000_000, // 调整为类型的最大预期大小（字节）
                is_fixed_size: false,
            };
        }
    };

    ($type:ident) => {
        use ic_stable_structures::storable::Bound;
        use ic_stable_structures::Storable;
        use std::borrow::Cow;

        impl Storable for $type {
            fn to_bytes(&self) -> Cow<[u8]> {
                let bytes = bincode::serialize(self).expect("Serialization failed");
                Cow::Owned(bytes)
            }

            fn from_bytes(bytes: Cow<[u8]>) -> Self {
                bincode::deserialize(&bytes).expect("Deserialization failed")
            }

            const BOUND: Bound = Bound::Bounded {
                max_size: 10_000_000, // 调整为类型的最大预期大小（字节）
                is_fixed_size: false,
            };
        }
    };
}

#[macro_export]
macro_rules! generate_generic_service_trait {
    ($x:ident) => {
            pub trait $x {
                type Output;

                fn create() -> Option<Self::Output>;

                fn is_exist(principal: Principal) -> bool;

                fn get(principal: Principal) -> Option<Self::Output>;

                fn delete(&self);
            }

    };
}

#[macro_export]
macro_rules! impl_error {
    ($entity:ident) => {
        impl Display for $entity {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let str=stringify!($entity).from();
                write!(f,str+":{:?}", self)
            }
        }
        impl Error for $entity {}
        impl From<$entity> for std::fmt::Error {
            fn from(_err: $entity) -> std::fmt::Error {
                std::fmt::Error
            }
        }
    };
}