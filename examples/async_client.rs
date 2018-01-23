extern crate kafka;
extern crate tokio_core;
extern crate futures;

use kafka::async_client::Client;
use std::net::SocketAddr;
use tokio_core::reactor::Core;
use kafka::protocol::ProduceRequest;
use futures::future::Future;
use kafka::compression::Compression;
use futures::future::join_all;

fn main() {
    let addr : SocketAddr = "0.0.0.0:9092".parse().unwrap();
    let mut core: Core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::connect(&addr, &handle)
        .and_then(|item: Client| {
            println!("Connected to kafka...");
            let all: Vec<_> = (0..10).into_iter().map(|_| item.produce()).collect();
            let produces = join_all(all);
            produces
        });
    core.run(client).unwrap();
}