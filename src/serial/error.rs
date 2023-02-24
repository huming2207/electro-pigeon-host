use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerialError {
    #[error("Packet CRC mismatched, expected {expected:02x}, actual {actual:02x}")]
    PacketCrcMismatch {
        expected: u16,
        actual: u16
    },

    #[error("Unknown packet {0}")]
    UnknownPacket(u8),

    #[error("Unsupported packet structure")]
    PacketStructure,

    #[error("Packet is too short, only {0} bytes while {1} expected")]
    PacketTooShort(usize, usize),

    #[error("Serial port error: {0}")]
    Port(#[from] tokio_serial::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error)
}