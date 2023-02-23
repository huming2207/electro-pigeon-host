use tokio_serial::SerialPortBuilderExt;
use tokio_util::codec::Decoder;
use crate::serial::codec::StatefulSlipCodec;
use crate::serial::error::SerialError;
use tokio_stream::StreamExt;
use futures::sink::SinkExt;

pub struct RedShoreSerial {
    codec: StatefulSlipCodec,
}

impl RedShoreSerial {
    pub async fn new(dev: String, baud: u32) -> Result<(), SerialError> {
        let mut port = tokio_serial::new(dev, baud).open_native_async()?;

        #[cfg(unix)]
        port.set_exclusive(true)?;

        let mut reader = StatefulSlipCodec.framed(port);

        while let Some(output) = reader.next().await {
            let buf = output?;
            // reader.send(buf).await?;
        }

        Ok(())
    }
}