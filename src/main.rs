extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};
use tokio_proto::pipeline::{ServerProto, ClientProto};
use tokio_service::Service;
use tokio_proto::{TcpServer, TcpClient};
use futures::{future, Future, BoxFuture};

pub enum RequestType {
    Request,
    Response
}

pub struct ParameterPair {
    key: String,
    value: String,
}

pub struct ParameterBag {
    bag: Vec<ParameterPair>,
}

pub struct HttpMessage {
    headers: Vec<ParameterBag>,
    body: Vec<u8>,
    message_type: RequestType,
}

pub struct LineCodec {
    body: Vec<u8>
}

pub struct HttpProto;
pub struct Echo;


impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        // if (buf.ends_with(&[13, 10, 13, 10])) {
        //     Ok(None)
        // } else {
        //     match str::from_utf8(buf) {
        //         Ok(s) => {
        //             self.body.append(&mut buf.to_vec());

        //             Ok(Some(s.to_string()))
        //         },
        //         Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
        //     }
        // }

        println!("{:?}", buf);

        let size = buf.len();
        let part = buf.split_to(size);

        if (part.len() < 1) {
            Ok(None)
        } else {
            match str::from_utf8(&part) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }
        }


    //    if let Some(i) = buf.iter().position(|&b| b == b'\r') {
    //        // remove the serialized frame from the buffer.
    //        let line = buf.split_to(i);

    //        println!("{:?} {:?}", str::from_utf8(&buf), buf.ends_with(&[13, 10, 13, 10]));

    //        // Also remove the '\n'
    //        buf.split_to(1);

    //        // Turn this data into a UTF string and return it in a Frame.
    //        match str::from_utf8(&line) {
    //            Ok(s) => Ok(Some(s.to_string())),
    //            Err(_) => Err(io::Error::new(io::ErrorKind::Other,
    //                                         "invalid UTF-8")),
    //        }
    //    } else {
    //        Ok(None)
    //    }
    }

}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        println!("{:?}", msg);

        buf.extend("HTTP/1.1 200 OK\r\nDate: Tue, 30 May 2017 05:34:07 GMT\r\nContent-Type: text/html\r\nContent-Length: 9\r\n\r\n<b>Ok</b>".as_bytes());

        Ok(())
    }

}

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for HttpProto {
    /// For this protocol style, `Request` matches the `Item` type of the codec's `Encoder`
    type Request = String;

    /// For this protocol style, `Response` matches the `Item` type of the codec's `Decoder`
    type Response = String;

    /// A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, LineCodec>;

    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec { body: Vec::new() }))
    }
}

impl Service for Echo {
    // These types must match the corresponding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        // In this case, the response is immediate.
        future::ok(req).boxed()
    }
}

fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(HttpProto, addr);

    // let client = TcpClient::new(LineProto);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(Echo));
}
