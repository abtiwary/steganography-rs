//! encode a message into a given slice of bytes, where each source byte is
//! partially encoded into the least significant bits of the destination bytes

// magic start: 171, 176
// magic end: 186, 187

use anyhow::{anyhow, Result};

static  MAGIC_START: &[u8] = &[171u8, 176u8];
static MAGIC_END: &[u8] = &[186u8, 187u8];

#[non_exhaustive]
#[derive(Debug)]
pub struct Encoder<'source, 'message> {
    source_bytes: &'source mut [u8],
    message: &'message [u8],
}

impl <'source, 'message> Encoder<'source, 'message> {

    pub fn new(source_bytes: &'source mut [u8], message: &'message [u8]) -> Result<Self> {
        Ok(
            Encoder{
                source_bytes,
                message
            }
        )
    }

    pub fn encode(&mut self) -> Result<usize> {
        if  self.source_bytes.len() < ((self.message.len() + 4) * 4) {
            return Err(anyhow!("not enough bytes available"));
        }

        let mut source_idx = 0usize;
        let mut message_idx = 0usize;
        let mut bytes_encoded = 0usize;

        let mut magic_end_bytes_encoded = 0usize;

        loop {
            // get the bits to encode from the message
            let target_byte = match bytes_encoded {
                b if b < MAGIC_START.len() => { MAGIC_START[bytes_encoded] },
                
                b if b >= MAGIC_START.len() + self.message.len() => {
                    let target_byte = MAGIC_END[magic_end_bytes_encoded];
                    magic_end_bytes_encoded += 1;
                    target_byte
                },
                _ => { 
                    let target_byte = self.message[message_idx];
                    message_idx += 1; 
                    target_byte
                },
            };

            for i in 0usize..4usize {
                let mut source_byte = self.source_bytes[source_idx + i];

                // clear the last two bits
                source_byte &= 0b1111_1100;
                
                let mut op = target_byte & (0b1100_0000 >> i*2);
                op = op >> (8 - ((i+1)*2));
                
                source_byte |=  op; 

                self.source_bytes[source_idx + i] = source_byte;
            }

            source_idx += 4;
            bytes_encoded += 1;

            if bytes_encoded >= self.message.len() + 4 {
                break;
            }
        }

        Ok(bytes_encoded)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

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

    }


}

