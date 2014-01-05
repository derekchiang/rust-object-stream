# rust-object-stream

Directly send/receive objects over any `std::io::Stream`, with serialization/deserialization taken care of.

## Usage

```rust
#[deriving=(Encodable, Decodable)]
struct Message {
	content: ~str
}
let msg = Message{ content: "Hello" };
let stream = \* Create a stream, e.g. TcpStream *\;
let obj_stream = ObjectStream::new(stream);
obj_stream.send(msg);

// On the other end
let stream = \* Create a stream, e.g. TcpStream *\;
let obj_stream = ObjectStream::new(stream)
let msg = obj_stream::recv();
```
