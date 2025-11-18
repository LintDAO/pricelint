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
    ($type:ident <$genric:ident> $(,[$max_size:expr,$is_fixed_size:expr])?) => {
        impl<$genric> Storable for $type<$genric>
        where
            $genric: Serialize + for<'de> Deserialize<'de>, $genric: candid::CandidType
        {
            fn to_bytes(&self) -> Cow<[u8]> {
                Cow::Owned(Encode!(self).unwrap())
            }

            fn from_bytes(bytes: Cow<[u8]>) -> Self {
                Decode!(bytes.as_ref(), Self).unwrap()
            }

            const BOUND: Bound = Bound::Bounded {
                max_size: impl_storable!(@get max_size, $($max_size)?),
                is_fixed_size: impl_storable!(@get is_fixed_size, $($is_fixed_size)?),
            };


        }
    };


    ($type:ident <$genric1:ident,$genric2:ident> $(, [$max_size:expr, $is_fixed_size:expr])?) => {
        impl<$genric1,$genric2> Storable for $type<$genric1,$genric2>
        where
            $genric1: Serialize + for<'de> Deserialize<'de>,
            $genric2: Serialize + for<'de> Deserialize<'de>, $genric1: std::cmp::Ord+candid::CandidType, $genric2: candid::CandidType
        {
            fn to_bytes(&self) -> Cow<[u8]> {
                Cow::Owned(Encode!(self).unwrap())
            }

            fn from_bytes(bytes: Cow<[u8]>) -> Self {
                Decode!(bytes.as_ref(), Self).unwrap()
            }

            const BOUND: Bound = Bound::Bounded {
                 max_size: impl_storable!(@get max_size, $($max_size)?),
                 is_fixed_size: impl_storable!(@get is_fixed_size, $($is_fixed_size)?),
            };
        }
    };



    ($type:ident$(,[$max_size:expr,$is_fixed_size:expr])?) => {
        impl Storable for $type {
            fn to_bytes(&self) -> Cow<[u8]> {
                Cow::Owned(Encode!(self).unwrap())
            }

            fn from_bytes(bytes: Cow<[u8]>) -> Self {
                Decode!(bytes.as_ref(), Self).unwrap()
            }

            const BOUND: Bound = Bound::Bounded {
                 max_size: impl_storable!(@get max_size, $($max_size)?),
                 is_fixed_size: impl_storable!(@get is_fixed_size, $($is_fixed_size)?),
            };
        }
    };


    (@get max_size, $value:expr) => { $value };
    (@get max_size,) => { 10_000_000 };
    (@get is_fixed_size, $value:expr) => { $value };
    (@get is_fixed_size,) => { false };
}
