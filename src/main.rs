extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Default, PartialEq, Debug)]
struct Payload {
    kind: u8,
    value: u16,
}

fn run() -> io::Result<()> {
    let original_payload = Payload::default();
    let mut decoded_payload = Payload::default();

    let encoded_bytes = original_payload.encode()?;
    decoded_payload.decode(&encoded_bytes)?;

    assert_eq!(original_payload, decoded_payload);
    Ok(())
}

impl Payload {
    fn encode(&self) -> io::Result<Vec<u8>> {
        let mut bytes = vec![];
        bytes.write_u8(self.kind)?;
        bytes.write_u16::<LittleEndian>(self.value)?;
        Ok(bytes)
    }

    fn decode(& mut self, mut bytes: &[u8]) -> io::Result<()> {
        self.kind = bytes.read_u8()?;
        self.value = bytes.read_u16::<LittleEndian>()?;
        Ok(())
    }
}

fn main() {
    let _ = run();
}
