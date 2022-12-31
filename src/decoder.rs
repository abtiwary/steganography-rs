//! decode a message from a given slice of bytes, where each source byte is
//! partially encoded into the least significant bits of the destination bytes

// magic start: 171, 176
// magic end: 186, 187

use anyhow::{anyhow, Result};

static  MAGIC_START: &[u8] = &[171u8, 176u8];
static MAGIC_END: &[u8] = &[186u8, 187u8];

#[non_exhaustive]
#[derive(Debug)]
pub struct Decoder<'source> {
    source_bytes: &'source [u8],
}

impl <'source> Decoder<'source> {

    pub fn new(source_bytes: &'source [u8]) -> Result<Self> {
        Ok(
            Decoder{
                source_bytes
            }
        )
    }

    pub fn decode(&mut self) -> Result<Vec<u8>> {
        if  self.source_bytes.len() < (4 * 4) {
            return Err(anyhow!("not enough bytes available"));
        }

        let mut result: Vec<u8> = Vec::new();
        let mut found_magic_start: Vec<u8> = Vec::with_capacity(2);
        let mut found_magic_end: Vec<u8> = Vec::with_capacity(2);

        let mut source_idx = 0usize;
        let mut bytes_decoded = 0usize;
        let mut message_bytes_decoded = 0usize;
        let mut magic_start_bytes_decoded = 0usize;
        let mut magic_end_bytes_decoded = 0usize;

        loop {
            let mut message_byte = 0u8;

            for i in 0usize..4usize {
                let mut source_byte = self.source_bytes[source_idx + i];

                // get the last two bits
                source_byte &= 0b0000_0011;
                
                let op = source_byte << (8 - ((i+1)*2));

                message_byte |= op;
            }

            source_idx += 4;
            bytes_decoded += 1;

            match bytes_decoded {
                b if b <= MAGIC_START.len() => {
                    magic_start_bytes_decoded += 1;
                    found_magic_start.push(message_byte);

                    if b == MAGIC_START.len() {
                        if found_magic_start != MAGIC_START {
                            return Err(anyhow!("magic start not found!"));
                        }
                    }
                },
                _ => {
                    if message_byte == MAGIC_END[0] {
                        found_magic_end.push(message_byte);
                        magic_end_bytes_decoded += 1;
                    } else if message_byte == MAGIC_END[1] {
                        found_magic_end.push(message_byte);
                        magic_end_bytes_decoded += 1;
                        if found_magic_end != MAGIC_END {
                            return Err(anyhow!("magic end not found!"));
                        } else {
                            break;
                        }
                    } else {
                        result.push(message_byte);
                        message_bytes_decoded += 1;
                    }
                }
            }
        }

        println!("magic start bytes decoded: {}", magic_start_bytes_decoded);
        println!("message bytes decoded: {}", message_bytes_decoded);
        println!("magic end bytes decoded: {}", magic_end_bytes_decoded);
        
        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::encoder::Encoder;

    #[test]
    fn it_works() {
        let mut source = String::from("this is some source string to encode the message into");
        let message = String::from("four");

        unsafe {

        let mut enc = Encoder::new(
            source.as_bytes_mut(),
            message.as_bytes()
        ).unwrap();

        let byte_count = enc.encode().unwrap();
        println!("{:?}", byte_count);
        }

        // now try and decode
        let mut dec = Decoder::new(source.as_bytes()).unwrap();
        let dec_message = dec.decode().unwrap();

        println!("{:?}", &dec_message);
        
        assert_eq!(&dec_message[..], &message.as_bytes()[..]);

        println!("{:?}", String::from_utf8(dec_message).unwrap());

    }

}
