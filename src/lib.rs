#![no_std]
#![no_main]

use anyhow;
use byteorder::ByteOrder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub fn encode<Input: Serialize>(x: Input, output: &mut [u8]) -> usize {
    let mut buf: [u8; 1024] = [0; 1024];
    let n = serde_json_core::to_slice(&x, &mut buf).unwrap();
    let hash = crc32fast::hash(&buf[0..n]);
    byteorder::LittleEndian::write_u32(&mut buf[n..n + 4], hash);
    let len = cobs::encode(&buf[0..n + 4], output);
    output[len] = 0;
    return len + 1;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    CobsDecode,
    JsonDecode,
    CrcInvalid,
}

pub fn decode<T>(source: &[u8]) -> anyhow::Result<T, Error>
where
    T: DeserializeOwned,
{
    let mut dest: [u8; 1024] = [0; 1024];
    let len = cobs::decode(source, &mut dest).map_err(|e| Error::CobsDecode)?;
    let (t, _) =
        serde_json_core::from_slice::<T>(&dest[0..len - 4]).map_err(|e| Error::JsonDecode)?;

    let calc_hash = crc32fast::hash(&dest[0..len - 4]);
    let data_hash = u32::from_le_bytes(dest[len - 4..len].try_into().unwrap());

    if calc_hash != data_hash {
        return Err(Error::CrcInvalid);
    };

    return Ok(t);
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    struct TestStruct {
        x: i32,
        y: i32,
    }

    #[test]
    fn test_encode() {
        let test_struct = TestStruct { x: 10, y: 10 };
        let mut cobs_data: [u8; 1024] = [0; 1024];
        let n = encode(test_struct, &mut cobs_data);
        // println!("{:?}", &cobs_data[..n]);
    }

    #[test]
    fn test_roundtrip() {
        let test_struct = TestStruct { x: 10, y: 10 };
        let mut cobs_data: [u8; 1024] = [0; 1024];
        let n = encode(test_struct, &mut cobs_data);
        // println!("{:?}", &cobs_data[..n]);
        let res = decode::<TestStruct>(&cobs_data);
        // println!("{:?}", res);
        res.unwrap();
    }
}
