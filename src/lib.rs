//! Clients for comunicating with a [Kafka](http://kafka.apache.org/)
//! cluster.  These are:
//!
//! - `kafka::producer::Producer` - for sending message to Kafka
//! - `kafka::consumer::Consumer` - for retrieving/consuming messages from Kafka
//! - `kafka::client::KafkaClient` - a lower-level, general purpose client leaving
//!   you with more power but also more resposibility
//!
//! See module level documentation corresponding to each client individually.
#![recursion_limit="128"]
#![cfg_attr(feature = "nightly", feature(test))]

extern crate byteorder;
extern crate crc;
extern crate ref_slice;
extern crate fnv;
extern crate twox_hash;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate bytes;
extern crate futures;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate log;

#[cfg(feature = "security")]
extern crate openssl;

#[cfg(feature = "gzip")]
extern crate flate2;

#[cfg(feature = "snappy")]
extern crate snap;

#[cfg(all(test, feature = "nightly"))]
extern crate test;

pub mod error;
pub mod client;
mod client_internals;
pub mod consumer;
pub mod producer;
mod utils;
mod codecs;
pub mod protocol;
pub mod compression;
pub mod async_client;
pub mod proto;
pub mod codec;

pub use self::error::{Error, Result};
