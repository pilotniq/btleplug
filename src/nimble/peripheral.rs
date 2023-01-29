
use async_trait::async_trait;

use std::collections::{BTreeSet, HashMap};
use futures::stream::{Stream, StreamExt};

use crate::api::{
    self, AddressType, BDAddr, CharPropFlags, Characteristic, PeripheralProperties, Service,
    ValueNotification, WriteType,
};
use crate::{Error, Result};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PeripheralId( /* pub(crate) DeviceId */);

/// Implementation of [api::Peripheral](crate::api::Peripheral).
#[derive(Clone, Debug)]
pub struct Peripheral {
//    session: BluetoothSession,
//    device: DeviceId,
//    mac_address: BDAddr,
//    services: Arc<Mutex<HashMap<Uuid, ServiceInternal>>>,
}

#[async_trait]
impl api::Peripheral for Peripheral {
    fn id(&self) -> PeripheralId {
        // PeripheralId(self.device.to_owned())
        PeripheralId()
    }

    fn address(&self) -> BDAddr {
        BDAddr {}
        // self.mac_address
    }

    async fn properties(&self) -> Result<Option<PeripheralProperties>> {
        Ok(None)
        /*
        let device_info = self.device_info().await?;
        Ok(Some(PeripheralProperties {
            address: device_info.mac_address.into(),
            address_type: Some(device_info.address_type.into()),
            local_name: device_info.name,
            tx_power_level: device_info.tx_power,
            rssi: device_info.rssi,
            manufacturer_data: device_info.manufacturer_data,
            service_data: device_info.service_data,
            services: device_info.services,
        }))
         */
    }

    fn services(&self) -> BTreeSet<Service> {
        []
        /*
        self.services
            .lock()
            .unwrap()
            .values()
            .map(|service| service.into())
            .collect()
         */
    }

    async fn is_connected(&self) -> Result<bool> {
        Ok(false)
        /*
        let device_info = self.device_info().await?;
        Ok(device_info.connected)
         */
    }

    async fn connect(&self) -> Result<()> {
        // self.session.connect(&self.device).await?;
        Ok(())
    }

    async fn disconnect(&self) -> Result<()> {
        // self.session.disconnect(&self.device).await?;
        Ok(())
    }

    async fn discover_services(&self) -> Result<()> {
        /*
        let mut services_internal = HashMap::new();
        let services = self.session.get_services(&self.device).await?;
        for service in services {
            let characteristics = self.session.get_characteristics(&service.id).await?;
            services_internal.insert(
                service.uuid,
                ServiceInternal {
                    info: service,
                    characteristics: characteristics
                        .into_iter()
                        .map(|characteristic| (characteristic.uuid, characteristic))
                        .collect(),
                },
            );
        }
        *self.services.lock().unwrap() = services_internal;
         */
        Ok(())
    }

    async fn write(
        &self,
        characteristic: &Characteristic,
        data: &[u8],
        write_type: WriteType,
    ) -> Result<()> {
        /*
        let characteristic_info = self.characteristic_info(characteristic)?;
        let options = WriteOptions {
            write_type: Some(write_type.into()),
            ..Default::default()
        };
        Ok(self
            .session
            .write_characteristic_value_with_options(&characteristic_info.id, data, options)
            .await?)
             */
        Ok(None)
    }

    async fn read(&self, characteristic: &Characteristic) -> Result<Vec<u8>> {
    /*
        let characteristic_info = self.characteristic_info(characteristic)?;
        Ok(self
            .session
            .read_characteristic_value(&characteristic_info.id)
            .await?)
         */
        Ok(None)
    }

    async fn subscribe(&self, characteristic: &Characteristic) -> Result<()> {
        /*
        let characteristic_info = self.characteristic_info(characteristic)?;
        Ok(self.session.start_notify(&characteristic_info.id).await?)
         */
        Ok()
    }

    async fn unsubscribe(&self, characteristic: &Characteristic) -> Result<()> {
        /*
        let characteristic_info = self.characteristic_info(characteristic)?;
        Ok(self.session.stop_notify(&characteristic_info.id).await?)
        */
        Ok()
    }

    async fn notifications(&self) -> Result<Pin<Box<dyn Stream<Item = ValueNotification> + Send>>> {
/*
        let device_id = self.device.clone();
        let events = self.session.device_event_stream(&device_id).await?;
        let services = self.services.clone();
        Ok(Box::pin(events.filter_map(move |event| {
            ready(value_notification(event, &device_id, services.clone()))
        })))
         */
        Ok()
    }
}