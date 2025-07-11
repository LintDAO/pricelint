#[macro_export]
macro_rules! impl_error {
    ($entity:ident) => {
        impl Display for $entity {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let _entity_name = stringify!($entity);
                // write!(f, "[error type]:{} ,[result]:{:?}", entity_name, self)
                write!(f, "{:?}", self)
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
#[macro_export]
macro_rules! impl_storable {
    ($type:ident <$gen:ident >) => {
       impl<$genric> Storable for $type<$genric>
        where
            $genric: Serialize + for<'de> Deserialize<'de>,
        {
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
