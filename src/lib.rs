//! A proxy actor and infrastructure for asynchronous and clear cache interaction for Actix actor and Actix-web frameworks.
//!
//! # A tour of actix-cache
//!
//! This crate consist of three main part:
//! * [CacheActor] actix actor.
//! * [Backend] trait and its implementation ([RedisBackend]).
//! * [Cacheable] trait.
//!
//! ## Features
//! * Async/Sync cache backend support.
//! * [Dogpile] effect prevention.
//! * Stale cache mechanics.
//! * Automatic cache key generation.
//! * Detailed Prometheus metrics out of the box.
//!
//! ## Feature flags
//! * derive - Support for [Cacheable] trait derive macros.
//! * metrics - Support for Prometheus metrics.
//!
//! ## Example
//! First of all, you should derive [Cacheable] trait for your actix Message:
//!
//! > **_NOTE:_** Default cache key implementation based on serde_qs crate
//! > and have some [restrictions](https://docs.rs/serde_qs/latest/serde_qs/#supported-types).
//!
//!
//! ```rust
//! use actix::prelude::*;
//! use actix_cache::Cacheable; // With features=["derive"]
//! use actix_derive::Message;
//! use serde::{Deserialize, Serialize};
//! # struct Pong;
//!
//! #[derive(Message, Cacheable, Serialize)]
//! #[rtype(result = "Result<Pong, ()>")]
//! struct Ping {
//!     id: i32,
//! }
//! ```
//! Or implement that trait manually:
//!
//! ```rust
//! # use actix_cache::{Cacheable, CacheError};
//! # struct Ping { id: i32 }
//! impl Cacheable for Ping {
//!     fn cache_message_key(&self) -> Result<String, CacheError> {
//!         Ok(format!("{}::{}", self.cache_key_prefix(), self.id))
//!     }
//!
//!     fn cache_key_prefix(&self) -> String { "Ping".to_owned() }
//! }
//! ```
//! Next step is to instantiate [CacheActor] with default or selected by feature backend:
//!
//! ```rust
//! # use actix::prelude::*;
//! use actix_cache::{Cache, CacheError, RedisBackend};
//!
//! #[actix_rt::main]
//! async fn main() -> Result<(), CacheError> {
//!     let cache = Cache::new()
//!         .await?
//!         .start();
//! #   Ok(())
//! }
//! ```
//! Or you can instantiate [CacheActor] with any other backend manually 
//! (for additional information you can backend examples):
//!
//! ```rust
//! # use actix::prelude::*;
//! use actix_cache::{CacheActor, CacheError, RedisBackend};
//!
//! #[actix_rt::main]
//! async fn main() -> Result<(), CacheError> {
//!     let backend = RedisBackend::new()
//!         .await
//!         .map_err(|err| CacheError::BackendError(err.into()))?
//!         .start();
//!     let cache = CacheActor::builder()
//!         .build(backend)
//!         .start();
//! #   Ok(())
//! }
//! ```
//!
//! And the last step is using cache in your code (actix-web handler for example).
//! This full example and other examples you can see on [github.com](https://github.com/rambler-digital-solutions/actix-cache/blob/master/examples/actix_web.rs)
//!
//! ```rust
//! # use actix::prelude::*;
//! # use actix_web::{web, App, HttpResponse, HttpServer, Responder};
//! # use actix_cache::{Cache, RedisBackend, Cacheable};
//! # use serde::Serialize;
//! #
//! # struct FibonacciActor;
//! #
//! # impl Actor for FibonacciActor { type Context = Context<Self>; }
//! #
//! # #[derive(Message, Cacheable, Serialize)]
//! # #[rtype(result = "u64")]
//! # struct GetNumber {
//! #     number: u8
//! # }
//! #
//! # impl Handler<GetNumber> for FibonacciActor {
//! #     type Result = <GetNumber as Message>::Result;
//! #
//! #     fn handle(&mut self, msg: GetNumber, _ctx: &mut Self::Context) -> Self::Result {
//! #         42
//! #     }
//! # }
//! #
//! async fn index(
//!     fib: web::Data<Addr<FibonacciActor>>,
//!     cache: web::Data<Addr<Cache>>
//! ) -> impl Responder {
//!     let query = GetNumber { number: 40 };
//!     let number = cache
//!         .send(query.into_cache(&fib))
//!         .await
//!         .unwrap()
//!         .unwrap();
//!     HttpResponse::Ok().body(format!("Generate Fibonacci number {}", number))
//! }
//! ```
//!
//! ## Backend implementations
//!
//! At this time supported or planned next cache backend implementation:
//! - [x] Redis backend
//! - [ ] In-memory backend
//!
//! But you are welcome to add your own implementation of custom backend.
//! All you need are define new actix actor struct and implement `actix::Handle` trait for next
//! `Message`:
//!
//! * [Get]
//! * [Set]
//! * [Delete]
//! * [Lock]
//!
//! [CacheActor]: actor/struct.CacheActor.html
//! [Cacheable]: cache/trait.Cacheable.html
//! [Backend]: ../actix_cache_backend/trait.Backend.html
//! [RedisBackend]: ../actix_cache_redis/actor/struct.RedisActor.html
//! [Get]: dev/struct.Get.html
//! [Set]: dev/struct.Set.html
//! [Delete]: dev/struct.Delete.html
//! [Lock]: dev/struct.Lock.html
//! [Dogpile]: https://www.sobstel.org/blog/preventing-dogpile-effect/
#![warn(missing_docs)]
pub mod actor;
pub mod cache;
pub mod dev;
pub mod error;
#[cfg(feature = "metrics")]
pub mod metrics;
pub mod response;

pub use actor::{CacheActor, CacheBuilder};
pub use cache::{Cacheable, QueryCache};
pub use error::CacheError;

pub use actix_cache_redis::RedisBackend;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use serde_qs;

/// Default type alias with RedisBackend.
/// You can disable it or define it manually in your code.
#[cfg(feature = "redis")]
pub type Cache = CacheActor<RedisBackend>;
