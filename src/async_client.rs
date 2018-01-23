use tokio_proto::multiplex::{ClientService};
use proto::Proto;
use tokio_core::net::TcpStream;
use tokio_service::Service;
use std::io;
use futures::Future;
use std::net::SocketAddr;
use tokio_proto::TcpClient;
use tokio_core::reactor::Handle;
use codecs::{FromByte, ToByte};
use protocol::produce::{ProduceResponse, ProduceRequest};
use bytes::BytesMut;
use compression::Compression;
use std::io::Cursor;
use futures::IntoFuture;
use byteorder::{ByteOrder, BigEndian, WriteBytesExt};
pub struct Client
{
    inner: ClientService<TcpStream, Proto>,
}

impl Client {

    pub fn connect(addr: &SocketAddr, handle: &Handle) -> Box<Future<Item = Client, Error = io::Error>>  {
        let ret = TcpClient::new(Proto)
            .connect(addr, handle)
            .map(|client_service| {
                println!("connected");
                Client {
                    inner: client_service
                }
            });

        Box::new(ret)
    }

    pub fn produce(&self) -> Box<Future<Item = ProduceResponse, Error = ::std::io::Error> + 'static > {
        println!("produce executed");
        let mut produce_request = ProduceRequest::new(0, 2000, 0, "", Compression::NONE);
        produce_request.add("test", 0, None, Some("hello from kafka-rust async client".as_bytes()));
        let mut req = Vec::new();
        req.extend_from_slice(&[0,0,0,0]);
        produce_request.encode(&mut req).unwrap();
        let size = req.len() as i32 - 4;
        size.encode(&mut &mut req[..]).unwrap();
        println!("request serialized");
        let res = self
            .call(req)
            .and_then(|res| {
                println!("request recieved");
                let mut cursor = Cursor::new(res);
                Ok(ProduceResponse::decode_new(&mut cursor).unwrap())
            });
        Box::new(res)
    }
}

impl Service for Client {
    type Request = Vec<u8>;
    type Response = BytesMut;
    type Error = ::std::io::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        Box::new(self.inner.call(req))
    }
}