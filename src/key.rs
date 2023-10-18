use std::borrow::Cow;

/// Implementation requirements:
/// 1. Must give the same results for identical keys.
/// 2. Must map a key instance to a string in a unique way (should not allow two different keys
///    to produce identical strings), without collisions.
pub trait ExternalKey {
    fn external_key(&self) -> Cow<'_, str>;
}

impl ExternalKey for String {
    fn external_key(&self) -> Cow<'_, str> {
        self.into()
    }
}

impl ExternalKey for str {
    fn external_key(&self) -> Cow<'_, str> {
        self.into()
    }
}

macro_rules! impl_external_key {
    ($( $t:ty ),+) => {
        $(
            impl_external_key!{@single, $t}
        )+
    };
    (@single, $t:ty) => {
        impl ExternalKey for $t {
            fn external_key(&self) -> Cow<'_, str> {
                self.to_string().into()
            }
        }
    };
}

impl_external_key!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);
