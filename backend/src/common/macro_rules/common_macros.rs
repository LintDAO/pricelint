#[macro_export]
macro_rules! map_get {
    ($map:expr ,$key: expr) => {
        $map.with(|map| {
            let mut value = map.borrow_mut().get($key);
            value
        })
    };
    (ref $map:expr, $key:expr) => {{
        $map.with(|map| map.deref_mut().borrow_mut().get($key))
    }};
    ($map:expr, $key:expr,$t:lifetime) => {
        $map.with(|map| map.borrow::<$t>().get($key))
    };
}
#[macro_export]
macro_rules! map_insert {
    ($map: expr,$key: expr,$data: expr) => {
        $map.with(|map| map.borrow_mut().insert($key, $data))
    };
    (ref $map:expr, $key:expr) => {{
        $map.with(|map| map.deref_mut().borrow_mut().insert($key, $data))
    }};
}
macro_rules! map_remove {
    ($map: expr,$key: expr) => {
        $map.with(|map| map.borrow_mut().remove($key))
    };
    (ref $map:expr, $key:expr) => {{
        $map.with(|map| map.deref_mut().borrow_mut().remove($key))
    }};
}

#[macro_export]
macro_rules! impl_storable {
    ($type:ident <$genric:ident >) => {
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

#[macro_export]
macro_rules! impl_error {
    ($entity:ident) => {
        impl Display for $entity {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let entity_name = stringify!($entity);
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
