use std::io;

use bytes::{Buf, BytesMut};
use memmem::{Searcher, TwoWaySearcher};
use tokio_util::codec::Decoder;

const FRAME_START: [u8; 2] = [0x42, 0x4d];

pub struct Pms5003Codec<'a> {
    searcher: TwoWaySearcher<'a>,

    #[cfg(target_os = "macos")]
    frames_skipped: u8,
}

impl<'a> Pms5003Codec<'a> {
    pub fn new() -> Self {
        Pms5003Codec {
            searcher: TwoWaySearcher::new(&FRAME_START),

            #[cfg(target_os = "macos")]
            frames_skipped: 0,
        }
    }
}

impl<'a> Decoder for Pms5003Codec<'a> {
    type Item = Pms5003Frame;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.searcher.search_in(src) {
            None => Ok(None),
            Some(index) => {
                if index > 0 {
                    src.advance(index);
                }

                #[cfg(target_os = "macos")]
                if self.frames_skipped < 2 {
                    src.advance(2);
                    self.frames_skipped += 1;
                    return Ok(None);
                }

                if src.len() < 32 {
                    return Ok(None);
                }

                let frame = Pms5003Frame::from_raw(src[0..32].try_into().unwrap())?;
                src.advance(32);
                Ok(Some(frame))
            }
        }
    }
}

#[derive(Debug)]
pub struct Pms5003Frame {
    /// PM1.0 concentration in µg/m³, corrected for standard atmosphere
    pub pm10: u16,

    /// PM25 concentration in µg/m³, corrected for standard atmosphere
    pub pm25: u16,

    /// PM10 concentration in µg/m³, corrected for standard atmosphere
    pub pm100: u16,

    /// PM1.0 concentration in µg/m³, in current atmosphere
    pub pm10_atmos: u16,

    /// PM25 concentration in µg/m³, in current atmosphere
    pub pm25_atmos: u16,

    /// PM10 concentration in µg/m³, in current atmosphere
    pub pm100_atmos: u16,

    /// Number of >0.3µm particles per 0.1L
    pub pm03_count: u16,

    /// Number of >0.5µm particles per 0.1L
    pub pm05_count: u16,

    /// Number of >1.0µm particles per 0.1L
    pub pm10_count: u16,

    /// Number of >2.5µm particles per 0.1L
    pub pm25_count: u16,

    /// Number of >5.0µm particles per 0.1L
    pub pm50_count: u16,

    /// Number of >10.0µm particles per 0.1L
    pub pm100_count: u16,
}

impl Pms5003Frame {
    pub fn from_raw(raw: &[u8; 32]) -> Result<Self, std::io::Error> {
        if raw[0] != 0x42 && raw[1] != 0x4d {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Frame header invalid",
            ));
        }

        let received_checksum = u32::from_be_bytes([0, 0, raw[30], raw[31]]);
        let calculated_checksum: u32 = raw[..30]
            .iter()
            .map(|byte| u32::from_be_bytes([0, 0, 0, byte.clone()]))
            .sum();

        if received_checksum != calculated_checksum {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Received checksum ({}) did not match calculated checksum ({})",
                    received_checksum, calculated_checksum
                ),
            ));
        }

        let frame = Pms5003Frame {
            pm10: u16::from_be_bytes([raw[4], raw[5]]),
            pm25: u16::from_be_bytes([raw[6], raw[7]]),
            pm100: u16::from_be_bytes([raw[8], raw[9]]),
            pm10_atmos: u16::from_be_bytes([raw[10], raw[11]]),
            pm25_atmos: u16::from_be_bytes([raw[12], raw[13]]),
            pm100_atmos: u16::from_be_bytes([raw[14], raw[15]]),
            pm03_count: u16::from_be_bytes([raw[16], raw[17]]),
            pm05_count: u16::from_be_bytes([raw[18], raw[19]]),
            pm10_count: u16::from_be_bytes([raw[20], raw[21]]),
            pm25_count: u16::from_be_bytes([raw[22], raw[23]]),
            pm50_count: u16::from_be_bytes([raw[24], raw[25]]),
            pm100_count: u16::from_be_bytes([raw[26], raw[27]]),
        };

        Ok(frame)
    }
}
