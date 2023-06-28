mod characteristic_flags;
mod constants;
mod error;
mod events;
mod ffi;
mod into_bool;
mod into_cbuuid;
mod peripheral_manager;

use async_trait::async_trait;
use uuid::Uuid;

use self::peripheral_manager::PeripheralManager;
use crate::peripheral::PeripheralServer;
use crate::{gatt::service::Service, Error};

pub struct Peripheral {
    peripheral_manager: PeripheralManager,
}

impl Peripheral {
    #[allow(clippy::new_ret_no_self)]
    async fn new() -> Result<Self, Error> {
        Ok(Self {
            peripheral_manager: PeripheralManager::new(),
        })
    }
}

#[async_trait]
impl PeripheralServer for Peripheral {
    async fn is_powered(&self) -> Result<bool, Error> {
        Ok(self.peripheral_manager.is_powered())
    }

    async fn register_gatt(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn unregister_gatt(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn start_advertising(self: &Self, name: &str, uuids: &[Uuid]) -> Result<(), Error> {
        self.peripheral_manager.start_advertising(name, uuids);
        Ok(())
    }

    async fn stop_advertising(&self) -> Result<(), Error> {
        self.peripheral_manager.stop_advertising();
        Ok(())
    }

    async fn is_advertising(&self) -> Result<bool, Error> {
        Ok(self.peripheral_manager.is_advertising())
    }

    fn add_service(&self, service: &Service) -> Result<(), Error> {
        self.peripheral_manager.add_service(service);
        Ok(())
    }
}
