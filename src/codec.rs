use tokio_proto::multiplex::RequestId;
use tokio_io::codec::{Decoder, Encoder};
use codecs::{FromByte, ToByte};
use bytes::BytesMut;
use std::io;
use std::marker::PhantomData;
use protocol::produce::ProduceRequest;
use protocol::produce::ProduceResponse;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt, WriteBytesExt};
use std::io::{Cursor};

pub struct Codec {
    incoming_response_size: usize,
}

impl Codec {
    pub fn new() -> Self {
        Codec {
            incoming_response_size: 0,
        }
    }
}

impl Encoder for Codec {
    type Item = (RequestId, Vec<u8>);
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
       let (request_id, mut request) = item;
       use bytes::BufMut;
        println!("request: {:?}", request);
        dst.put_slice(&request);
        println!("request sended");
       Ok(())
    }
}

impl Decoder for Codec {
    type Item = (RequestId, BytesMut);
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        println!("src: {:?}", src);
        if src.len() < 4 {
            println!("src.len() < 4");
            Ok(None)
        } else if src.len() == 4 {
            println!("src.len() == 4");
            let raw_response_size = BigEndian::read_i32(src);
            self.incoming_response_size = (raw_response_size as usize) + 4;
            Ok(None)
        } else if src.len() > self.incoming_response_size {
            println!("src.len() >= 4");
            let mut res = src.split_to(self.incoming_response_size);
            let req_id = BigEndian::read_i32(&mut &mut res[4..8]);
            Ok(Some((req_id as u64, res)))
        } else {
            println!("other");
            Ok(None)
        }
    }
}