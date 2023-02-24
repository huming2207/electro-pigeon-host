use std::io;

use bytes::{BytesMut, Buf, BufMut};
use tokio_util::codec::{Decoder, Encoder};

pub struct StatefulSlipCodec;

pub const SSLIP_START: u8 = 0xa5;
pub const SSLIP_END: u8 = 0xc0;
pub const SSLIP_ESC: u8 = 0xdb;
pub const SSLIP_ESC_END: u8 = 0xdc;
pub const SSLIP_ESC_ESC: u8 = 0xdd;
pub const SSLIP_ESC_START: u8 = 0xde;

impl Decoder for StatefulSlipCodec {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 2 {
            return Ok(None); // A SSLIP packet must be longer than 2 bytes - continue to read if that's not enough anyway
        }

        // Search for start mark (0xA5)
        let start_mark = src.as_ref().iter().position(|b| *b == SSLIP_START);
        
        if start_mark.is_none() {
            return Ok(None); // If there's no start mark found, return None to continue
        } else if let Some(start_idx) = start_mark {
            src.advance(start_idx);
        } else {
            return Ok(None); // it will never reach to here??
        }

        // Search for end mark (0xC0)
        let end_mark = src.as_ref().iter().position(|b| *b == SSLIP_END);

        if end_mark.is_none() {
            return Ok(None); // Same as above, if it's not ended, proceed reading
        } else if let Some(end_idx) = end_mark {
            let pkt = src.split_to(end_idx + 1); // +1 to include the 0xC0 for further decoding
            let mut pkt_buf: Vec<u8> = Vec::new();
            pkt_buf.reserve(pkt.len());

            let mut is_esc = false;

            for byte in pkt {
                if is_esc {
                    match byte {
                        SSLIP_ESC_ESC => pkt_buf.push(SSLIP_ESC),
                        SSLIP_ESC_END => pkt_buf.push(SSLIP_END),
                        SSLIP_ESC_START => pkt_buf.push(SSLIP_START),
                        _ => {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Expected enscape byte but got 0x{:02x}]", byte)
                            ));
                        }
                    }
                
                    is_esc = false;
                } else {
                    match byte {
                        SSLIP_ESC => {
                            is_esc = true;
                        },
                        _ => pkt_buf.push(byte),
                    }
                }
            }

            return Ok(Some(pkt_buf));
        }

        return Ok(None);
    }
}

impl Encoder<&[u8]> for StatefulSlipCodec {
    type Error = io::Error;

    fn encode(&mut self, item: &[u8], dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u8(SSLIP_START);
        for byte in item {
            match *byte {
                SSLIP_ESC => {
                    dst.put_u8(SSLIP_ESC);
                    dst.put_u8(SSLIP_ESC_ESC);
                },
                SSLIP_START => {
                    dst.put_u8(SSLIP_ESC);
                    dst.put_u8(SSLIP_ESC_START);
                },
                SSLIP_END => {
                    dst.put_u8(SSLIP_ESC);
                    dst.put_u8(SSLIP_ESC_END);
                },
                _ => {
                    dst.put_u8(*byte);
                }
            }
        }

        dst.put_u8(SSLIP_END);
        Ok(())
    }
}

pub(crate) fn slice_to_u16le(buf: &[u8]) -> u16 {
    let arr: [u8; 2] = match buf.try_into() {
        Ok(arr) => arr,
        Err(_) => [0, 0],
    };

    u16::from_le_bytes(arr)
}

pub(crate) fn slice_to_u32le(buf: &[u8]) -> u32 {
    let arr: [u8; 4] = match buf.try_into() {
        Ok(arr) => arr,
        Err(_) => [0, 0, 0, 0],
    };

    u32::from_le_bytes(arr)
}

pub(crate) fn slice_to_u64le(buf: &[u8]) -> u64 {
    let arr: [u8; 8] = match buf.try_into() {
        Ok(arr) => arr,
        Err(_) => [0, 0, 0, 0, 0, 0, 0, 0],
    };

    u64::from_le_bytes(arr)
}