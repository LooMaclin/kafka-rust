use tokio_proto::multiplex::ClientProto;
use codec::Codec;
use tokio_io::codec::Framed;
use tokio_io::{AsyncWrite, AsyncRead};
use codecs::{ToByte, FromByte};
use std::io;
use std::marker::PhantomData;
use bytes::BytesMut;

pub struct Proto;

impl<T> ClientProto<T> for Proto
    where
        T: AsyncRead + AsyncWrite + 'static{

    type Request = Vec<u8>;
    type Response = BytesMut;
    type Transport = Framed<T, Codec>;
    type BindTransport = Result<Self::Transport, ::std::io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(Codec::new()))
    }
}