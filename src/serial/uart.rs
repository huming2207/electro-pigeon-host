use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tokio_util::codec::{Decoder, Framed};
use crate::serial::codec::StatefulSlipCodec;
use crate::serial::error::SerialError;
use tokio_stream::StreamExt;
use futures::sink::SinkExt;

pub struct RedShoreSerial {
    pub reader: Framed<SerialStream, StatefulSlipCodec>,
}

impl RedShoreSerial {
    pub async fn new(dev: String, baud: u32) -> Result<RedShoreSerial, SerialError> {
        let mut port = tokio_serial::new(dev, baud).open_native_async()?;

        #[cfg(unix)]
        port.set_exclusive(true)?;

        let reader = StatefulSlipCodec.framed(port);

        Ok(RedShoreSerial { reader })
    }

    pub async fn recv_packet(&mut self) -> Result<Option<Vec<u8>>, SerialError> {
        if let Some(output) = self.reader.next().await {
            return Ok(Some(output?));
        }

        Ok(None)
    }

    pub async fn send_packet(&mut self, buf: &[u8]) -> Result<(), SerialError> {
        Ok(self.reader.send(buf).await?)
    }
}