use thiserror::Error;

#[derive(Error, Debug)]
pub enum SerialError {
    #[error("CRC mismatched, expected {expected:02x}, actual {actual:02x}")]
    PacketCrcMismatch {
        expected: u16,
        actual: u16
    },

    #[error("Packet is too short, only {0} bytes")]
    PacketTooShort(u32),

    #[error("Serial port error: {0}")]
    Port(#[from] tokio_serial::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error)
}