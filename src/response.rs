use actix::Message;

use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

pub enum CachePolicy<T, U> {
    Cacheable(T),
    NonCacheable(U),
}

#[derive(Error, Debug)]
pub enum ResponseError {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error("Value is non cacheable")]
    NonCacheable,
}

pub trait CacheableResponse
where
    Self: Sized,
    Self::Cached: Serialize,
{
    type Cached;
    fn into_policy(self) -> CachePolicy<Self::Cached, Self>;
    fn from_cached(cached: Self::Cached) -> Self;
}
//
// impl CacheableResponse for i32 {
//     type Cached = i32;
//     fn into_policy(self) -> CachePolicy<Self::Cached, Self> {
//         CachePolicy::Cacheable(self)
//     }
//     fn from_cached(cached: Self::Cached) -> Self {
//         cached
//     }
// }

impl<I, E> CacheableResponse for Result<I, E> 
where
    I: Serialize + DeserializeOwned,
{
    type Cached = I;
    fn into_policy(self) -> CachePolicy<Self::Cached, Self> {
        match self {
            Ok(value) => CachePolicy::Cacheable(value),
            Err(ref error) => CachePolicy::NonCacheable(self),
        }
    }
    fn from_cached(cached: Self::Cached) -> Self {
        Ok(cached)
    }
}

macro_rules! CACHEABLE_RESPONSE_IMPL {
    ($type:ty) => {
        impl CacheableResponse for $type {
            type Cached = $type;
            fn into_policy(self) -> CachePolicy<Self::Cached, Self> {
                CachePolicy::Cacheable(self)
            }
            fn from_cached(cached: Self::Cached) -> Self {
                cached
            }
        }
    };
}

CACHEABLE_RESPONSE_IMPL!(());
CACHEABLE_RESPONSE_IMPL!(u8);
CACHEABLE_RESPONSE_IMPL!(u16);
CACHEABLE_RESPONSE_IMPL!(u32);
CACHEABLE_RESPONSE_IMPL!(u64);
CACHEABLE_RESPONSE_IMPL!(usize);
CACHEABLE_RESPONSE_IMPL!(i8);
CACHEABLE_RESPONSE_IMPL!(i16);
CACHEABLE_RESPONSE_IMPL!(i32);
CACHEABLE_RESPONSE_IMPL!(i64);
CACHEABLE_RESPONSE_IMPL!(isize);
CACHEABLE_RESPONSE_IMPL!(f32);
CACHEABLE_RESPONSE_IMPL!(f64);
CACHEABLE_RESPONSE_IMPL!(String);
CACHEABLE_RESPONSE_IMPL!(bool);


#[cfg(test)]
mod tests {
    use super::CacheableResponse;
    #[test]
    fn cacheable_response_result_serialize() {
        let res: Result<i32, i32> = Ok(42);
        assert_eq!(res.serialize(), Ok("42".to_owned()));
    }

    #[test]
    fn cacheable_response_result_deserialize() {
        let res = "42".to_owned();
        assert_eq!(Result::<i32, ()>::deserialize(res), Ok(Ok(42)));
    }
}