use crc::{Crc, CRC_16_DNP, CRC_16_KERMIT};
use crate::serial::error::SerialError;

use super::codec::{slice_to_u16le, slice_to_u32le, slice_to_u64le};

// See: https://github.com/mrhooray/crc-rs/issues/54#issuecomment-967742673
pub const UART_CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_DNP);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum SerialOpCode {
    Ack = 0,

    // Error
    ErrTimeout = 0xe0,
    ErrHeader = 0xe1,
    ErrChecksum = 0xe2,
    ErrNack = 0xff,
    ErrInternal = 0xfe,

    // Device setup
    Ping = 0x01, // From host
    DeviceInfo = 0x02, // To host
    ResetRadio = 0x03,
    ResetDevice = 0x04,
    LoraConfig = 0x05,
    LoraAdvConfig = 0x06,
    GfskConfig = 0x07,
    GfskAdvConfig = 0x08,

    // LoRa stuff
    LoraSendPacket = 0x10, // From host -> SUBGHZ -> Air
    LoraRecvPacket = 0x11, // To host

    // GFSK stuff
    GfskSendPacket = 0x20,
    GfskRecvPacket = 0x21,

    // Automatic transponder setting
}


#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PacketHeader {
    pub opcode: SerialOpCode,
    pub ctr: u8,
    pub crc: u16,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Ping {
    pub header: PacketHeader,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct DevInfo {
    pub header: PacketHeader,
    pub fw_ver: u32,
    pub mac: u64,
    pub uid: [u8; 12],
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct LoraConfig {
    pub header: PacketHeader,
    pub sf: u8,
    pub bw: u8,
    pub cr: u8,
    pub low_data_rate_opt: bool,
    pub sync_word: u8,
    pub freq_hz: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LoraTx {
    pub header: PacketHeader,
    pub tx_pwr: u8,
    pub timeout_ms: u32,
    pub preamble_cnt: u32,
    pub header_en: bool,
    pub crc_en: bool,
    pub invert_iq: bool,
    pub buf: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LoraRx {
    pub header: PacketHeader,
    pub pkt_rssi: u8,
    pub sig_rssi: u8,
    pub snr: u8,
    pub buf: Vec<u8>,
}

impl TryFrom<u8> for SerialOpCode {
    type Error = SerialError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SerialOpCode::Ack),
            0xe0 => Ok(SerialOpCode::ErrTimeout),
            0xe1 => Ok(SerialOpCode::ErrHeader),
            0xe2 => Ok(SerialOpCode::ErrChecksum),
            0xff => Ok(SerialOpCode::ErrNack),
            0xfe => Ok(SerialOpCode::ErrInternal),
            0x01 => Ok(SerialOpCode::Ping),
            0x02 => Ok(SerialOpCode::DeviceInfo),
            0x03 => Ok(SerialOpCode::ResetRadio),
            0x04 => Ok(SerialOpCode::ResetDevice),
            0x05 => Ok(SerialOpCode::LoraConfig),
            0x06 => Ok(SerialOpCode::LoraAdvConfig),
            0x07 => Ok(SerialOpCode::GfskConfig),
            0x08 => Ok(SerialOpCode::GfskAdvConfig),
            0x10 => Ok(SerialOpCode::LoraSendPacket),
            0x11 => Ok(SerialOpCode::LoraRecvPacket),
            0x20 => Ok(SerialOpCode::GfskSendPacket),
            0x21 => Ok(SerialOpCode::GfskRecvPacket),
            _ => Err(SerialError::UnknownPacket(value)),
        }
    }
}

impl TryFrom<&[u8]> for PacketHeader {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let opcode = SerialOpCode::try_from(value[0])?;
        let ctr = value[1];
        let crc = slice_to_u16le(&value[2..4]);

        let mut buf: Vec<u8> = Vec::from(value);
        buf[2] = 0;
        buf[3] = 0;

        let actual_crc = UART_CRC.checksum(&buf);

        if actual_crc != crc {
            return Err(SerialError::PacketCrcMismatch { expected: crc, actual: actual_crc });
        }

        Ok(PacketHeader { opcode, ctr, crc })
    }
}

impl TryFrom<&[u8]> for Ping {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 4 {
            return Err(SerialError::PacketTooShort(value.len(), 4));
        }

        let header = PacketHeader::try_from(value)?;
        Ok(Ping { header })
    }
}

impl TryFrom<&[u8]> for DevInfo {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 28 {
            return Err(SerialError::PacketTooShort(value.len(), 28));
        }

        let header = PacketHeader::try_from(value)?;
        let fw_ver = slice_to_u32le(&value[4..8]);
        let mac: u64 = slice_to_u64le(&value[8..16]);
        let uid: [u8; 12] = match value[16..value.len()].try_into() {
            Ok(val) => val,
            Err(_) => return Err(SerialError::PacketStructure),
        };

        Ok(DevInfo { header, fw_ver, mac, uid })
    }
}

impl TryFrom<&[u8]> for LoraConfig {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 13 {
            return Err(SerialError::PacketTooShort(value.len(), 13));
        }

        let header = PacketHeader::try_from(value)?;
        let sf = value[4];
        let bw = value[5];
        let cr = value[6];
        let low_data_rate_opt = value[7] > 0;
        let sync_word = value[8];
        let freq_hz = slice_to_u32le(&value[9..13]);
        Ok(LoraConfig { header, sf, bw, cr, low_data_rate_opt, sync_word, freq_hz })
    }
}

impl TryFrom<&[u8]> for LoraTx {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 17 {
            return Err(SerialError::PacketTooShort(value.len(), 17));
        }

        let header = PacketHeader::try_from(value)?;
        let tx_pwr = value[4];
        let timeout_ms = slice_to_u32le(&value[5..9]);
        let preamble_cnt = slice_to_u32le(&value[9..13]);
        let header_en = value[13] > 0;
        let crc_en = value[14] > 0;
        let invert_iq = value[15] > 0;
        let len = value[16] as usize;
        let buf: Vec<u8> = Vec::from(&value[17..len]);

        Ok(LoraTx { header, tx_pwr, timeout_ms, preamble_cnt, header_en, crc_en, invert_iq, buf })
    }
}

impl TryFrom<&[u8]> for LoraRx {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 17 {
            return Err(SerialError::PacketTooShort(value.len(), 17));
        }

        let header = PacketHeader::try_from(value)?;
        let pkt_rssi = value[4];
        let sig_rssi = value[5];
        let snr = value[6];
        let len = value[7] as usize;
        let buf: Vec<u8> = Vec::from(&value[8..len]);

        Ok(LoraRx { header, pkt_rssi, sig_rssi, snr, buf })
    }
}