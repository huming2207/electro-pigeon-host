use crate::serial::error::SerialError;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
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
    pub sf: u8,
    pub bw: u8,
    pub cr: u8,
    pub low_data_rate_opt: bool,
    pub sync_word: u8,
    pub freq_hz: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct LoraTx {
    pub tx_pwr: u8,
    pub timeout_ms: u32,
    pub preamble_cnt: u32,
    pub header_en: bool,
    pub crc_en: bool,
    pub invert_iq: bool,
    pub buf: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct LoraRx {
    pub pkt_rssi: u8,
    pub sig_rssi: u8,
    pub snr: u8,
    pub buf: Vec<u8>,
}

impl TryFrom<&[u8]> for PacketHeader {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for Ping {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for DevInfo {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for LoraConfig {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for LoraTx {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for LoraRx {
    type Error = SerialError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}