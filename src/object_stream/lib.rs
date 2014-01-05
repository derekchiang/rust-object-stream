#[crate_id = "object_stream"];

#[comment = "A wrapper around std::io::Stream that allows for sending/receiving objects directly."];
#[license = "MIT"];
#[crate_type = "lib"];

extern mod extra;

use std::io::{Reader, Writer, Stream, Decorator};
use std::io::mem::{MemWriter, BufReader};
use extra::serialize::{Encodable, Decodable};
use extra::json;


/**
A wrapper around std::io::Stream that allows for sending/receiving
objects directly.

Here is a complete example using TcpStream:

```rust
extern mod object_stream;
extern mod extra;

use std::io::{Listener, Acceptor};
use std::io::buffered::BufferedStream;
use std::io::net::tcp::{TcpStream, TcpListener};
use std::io::net::ip::SocketAddr;

use object_stream::ObjectStream;

#[deriving(Clone, Eq, Encodable, Decodable, ToStr)]
enum Salution {
    Hello(uint),
    Suppp(Sup)
}

#[deriving(Clone, Eq, Encodable, Decodable, ToStr)]
struct Sup {
    id: uint,
    name: ~str,
}

#[test]
fn test() {
    let s1 = Hello(10);
    let s2 = Suppp(Sup{
        id: 9,
        name: ~"oh yay",
    });

    let s1_clone = s1.clone();
    let s2_clone = s2.clone();

    let addr = from_str::<SocketAddr>("127.0.0.1:4001").unwrap();

    do spawn {
        let listener = TcpListener::bind(addr).unwrap();
        let mut acceptor = listener.listen().unwrap();
        let tcp_stream = acceptor.accept().unwrap();
        let mut stream = ObjectStream::new(BufferedStream::new(tcp_stream));
        stream.send::<Salution>(s1_clone);
        stream.send::<Salution>(s2_clone);
    }

    let tcp_stream = TcpStream::connect(addr).unwrap();
    let mut stream = ObjectStream::new(BufferedStream::new(tcp_stream));
    let s1_recv = stream.recv::<Salution>().unwrap();
    let s2_recv = stream.recv::<Salution>().unwrap();

    assert!(s1 == s1_recv);
    assert!(s2 == s2_recv);
}
```
*/
pub struct ObjectStream<T> {
    stream: T,
}

impl<T: Stream> ObjectStream<T> {
    pub fn new(stream: T) -> ObjectStream<T> {
        ObjectStream{
            stream: stream,
        }
    }

    /// Send an object.
    pub fn send<'a, U: Encodable<json::Encoder<'a>>>(&mut self, obj: U) {
        let mut mem_writer = MemWriter::new();

        // Encode the object
        let mut encoder = json::Encoder::new(&mut mem_writer as &mut Writer);
        obj.encode(&mut encoder);
        let bytes = mem_writer.inner();

        // Send the length of the object
        let len = bytes.len();
        self.stream.write_le_uint(len);

        // Send the object itself
        self.stream.write(bytes);
        self.stream.flush();
    }

    /// Receive an object.
    pub fn recv<'a, U: Decodable<json::Decoder>>(&mut self) -> Result<U, ~str> {
        // Read the length of the object
        let len = self.stream.read_le_uint();
        // Read the object itself
        let bytes = self.stream.read_bytes(len);

        // Decode
        let mut reader = BufReader::new(bytes);
        let j = match json::from_reader(&mut reader as &mut Reader) {
            Ok(j) => j,
            Err(err) => return Err(err.to_str()),
        };
        let mut decoder = json::Decoder::new(j);
        let obj = Decodable::decode(&mut decoder);

        return Ok(obj);
    }
}