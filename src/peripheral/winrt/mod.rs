mod error;
mod guid;
mod peripheral_manager;

use self::peripheral_manager::PeripheralManager;
use crate::gatt::service::Service;
use crate::peripheral::PeripheralServer;
use crate::Error;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
pub struct Peripheral {
    peripheral_manager: PeripheralManager,
}

impl Peripheral {
    pub async fn new() -> Result<Self, Error> {
        Ok(Self {
            peripheral_manager: PeripheralManager::new(),
        })
    }
}

#[async_trait]
impl PeripheralServer for Peripheral {
    async fn is_powered(&self) -> Result<bool, Error> {
        Ok(self.peripheral_manager.is_powered().await?)
    }

    async fn register_gatt(&self) -> Result<(), Error> {
        todo!()
    }

    async fn unregister_gatt(&self) -> Result<(), Error> {
        todo!()
    }

    async fn start_advertising(&self, name: &str, uuids: &[Uuid]) -> Result<(), Error> {
        todo!()
    }

    async fn stop_advertising(&self) -> Result<(), Error> {
        todo!()
    }

    async fn is_advertising(&self) -> Result<bool, Error> {
        todo!()
    }

    async fn add_service(&self, service: &Service) -> Result<(), Error> {
        todo!()
    }
}
