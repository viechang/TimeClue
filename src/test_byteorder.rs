#![allow(dead_code)]
mod test_byteorder {
    extern crate byteorder;

    use self::byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
    use std::io;


    #[derive(Default, PartialEq, Debug)]
    pub struct Payload {
        kind: u8,
        value: u16,
    }


    impl Payload {
        pub fn encode(&self) -> io::Result<Vec<u8>> {
            let mut bytes = vec![];
            bytes.write_u8(self.kind)?;
            bytes.write_u16::<LittleEndian>(self.value)?;
            Ok(bytes)
        }

        pub fn decode(&mut self, mut bytes: &[u8]) -> io::Result<()> {
            self.kind = bytes.read_u8()?;
            self.value = bytes.read_u16::<LittleEndian>()?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::test_byteorder::*;

    #[test]
    fn test_run() {
        let original_payload = Payload::default();
        let mut decoded_payload = Payload::default();

        let encoded_bytes = original_payload.encode().unwrap();
        decoded_payload.decode(&encoded_bytes).unwrap();

        assert_eq!(original_payload, decoded_payload);
    }
}

