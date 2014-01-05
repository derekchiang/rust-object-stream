#[crate_id = "object_stream"];

#[comment = "A wrapper around std::io::Stream that allows for sending/receiving objects directly."];
#[license = "MIT"];
#[crate_type = "lib"];

extern mod extra;

use std::io::{Reader, Writer, Stream, Decorator};
use std::io::mem::{MemWriter, BufReader};
use extra::serialize::{Encodable, Decodable};
use extra::json;

pub struct ObjectStream<T> {
    stream: T,
}

impl<T: Stream> ObjectStream<T> {
    pub fn new(stream: T) -> ObjectStream<T> {
        ObjectStream{
            stream: stream,
        }
    }

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