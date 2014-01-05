# rust-object-stream

Directly send/receive objects over a `std::io::Stream`, with serialization/deserialization taken care of.

## Build

1. Clone this repo.
2. `rustpkg build object_stream`

To build the docs:

1. `rustdoc src/object_stream/lib.rs`

## Usage

Here is a complete example:

```rust
extern mod object_stream;
extern mod extra;

use std::io::{Listener, Acceptor};
use std::io::buffered::BufferedStream;
use std::io::net::tcp::{TcpStream, TcpListener};
use std::io::net::ip::SocketAddr;

use object_stream::ObjectStream;

#[deriving(Clone, Eq, Encodable, Decodable)]
enum Salution {
    Hello(uint),
    Suppp(Sup)
}

#[deriving(Clone, Eq, Encodable, Decodable)]
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
