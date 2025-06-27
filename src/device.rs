use btleplug::{
    Error as BtError,
    api::{Central as _, CentralEvent, Characteristic, Peripheral as _, ScanFilter, WriteType},
    platform::{Adapter, Peripheral},
};
use futures::stream::StreamExt;
use log::{debug, info};

pub struct LedDevice {
    peripheral: Peripheral,
    ctrl_char: Characteristic,
}
impl LedDevice {
    async fn peripheral_info(peripheral: &Peripheral) -> String {
        let Some(props) = peripheral.properties().await.ok().flatten() else {
            return format!("<unknown properties> ({})", peripheral.id());
        };
        let name = props
            .local_name
            .unwrap_or_else(|| "<unknown name>".to_string());
        let address = if props.address.into_inner() != [0; 6] {
            props.address.to_string()
        } else {
            peripheral.id().to_string()
        };
        format!("{} ({})", name, address)
    }
    pub async fn find_peripheral(
        adapter: &Adapter,
        predicate: impl AsyncFn(&Peripheral) -> Result<bool, BtError>,
    ) -> Result<Option<Peripheral>, BtError> {
        let mut events = adapter.events().await?;
        adapter.start_scan(ScanFilter::default()).await?;
        info!("Start scanning...");
        let peripheral = 'b: loop {
            let Some(event) = events.next().await else {
                return Ok(None);
            };
            if let CentralEvent::DeviceDiscovered(id) = event {
                let peripheral = adapter.peripheral(&id).await?;
                debug!(
                    "Device discovered: {}",
                    Self::peripheral_info(&peripheral).await
                );
                if predicate(&peripheral).await? {
                    info!("Device found: {}", Self::peripheral_info(&peripheral).await);
                    break 'b peripheral;
                }
            }
        };
        adapter.stop_scan().await?;
        debug!("Stop scan");
        Ok(Some(peripheral))
    }
    pub async fn from(
        peripheral: Peripheral,
        char_predicate: impl Fn(&Characteristic) -> bool,
    ) -> anyhow::Result<Self> {
        if !peripheral.is_connected().await? {
            peripheral.connect().await?;
        }
        info!("Peripheral connected!");
        peripheral.discover_services().await?;
        let chars = peripheral.characteristics();
        if log::max_level() >= log::LevelFilter::Debug {
            debug!("{} characteristics", chars.len());
            for c in &chars {
                debug!("- {c:?}");
            }
        }
        let Some(ctrl_char) = chars.into_iter().find(char_predicate) else {
            return Err(crate::Error::CharNotFound)?;
        };
        Ok(Self {
            peripheral,
            ctrl_char,
        })
    }
    pub async fn send_command<A: AsRef<[u8]>>(&self, payload: A) -> Result<(), BtError> {
        info!("Sending command...");
        debug!("Command payload: {:x?}", payload.as_ref());
        let res = self
            .peripheral
            .write(&self.ctrl_char, payload.as_ref(), WriteType::WithResponse)
            .await;
        if res.is_ok() {
            info!("Command sent!");
        }
        res
    }
    #[inline]
    pub async fn power_on(&self) -> Result<(), BtError> {
        self.send_command(&[0xcc, 0x23, 0x33]).await
    }
    #[inline]
    pub async fn power_off(&self) -> Result<(), BtError> {
        self.send_command(&[0xcc, 0x24, 0x33]).await
    }
    pub async fn set_rgb(&self, r: u8, g: u8, b: u8) -> Result<(), BtError> {
        self.send_command(&[0x56, r, g, b, 0x00, 0xF0, 0xAA]).await
    }
}
