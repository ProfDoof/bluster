mod adapter;
mod advertisement;
mod common;
mod connection;
mod constants;
mod error;
mod gatt;

use async_trait::async_trait;
use std::{string::ToString, sync::Arc};
use uuid::Uuid;

use self::{adapter::Adapter, advertisement::Advertisement, connection::Connection, gatt::Gatt};
use crate::peripheral::PeripheralServer;
use crate::{gatt::service::Service, Error};

#[derive(Debug)]
pub struct Peripheral {
    adapter: Adapter,
    gatt: Gatt,
    advertisement: Advertisement,
}

impl Peripheral {
    #[allow(clippy::new_ret_no_self)]
    pub async fn new() -> Result<Self, Error> {
        let connection = Arc::new(Connection::new()?);
        let adapter = Adapter::new(connection.clone()).await?;
        adapter.powered(true).await?;
        let gatt = Gatt::new(connection.clone(), adapter.object_path.clone());
        let advertisement = Advertisement::new(connection, adapter.object_path.clone());

        Ok(Peripheral {
            adapter,
            gatt,
            advertisement,
        })
    }

    pub async fn get_alias(&self) -> Result<String, Error> {
        self.adapter.get_alias().await
    }

    pub async fn set_alias(&self, alias: &str) -> Result<(), Error> {
        self.adapter.set_alias(alias).await
    }
}

#[async_trait]
impl PeripheralServer for Peripheral {
    async fn is_powered(&self) -> Result<bool, Error> {
        self.adapter.is_powered().await
    }

    async fn register_gatt(&self) -> Result<(), Error> {
        self.gatt.register().await
    }

    async fn unregister_gatt(&self) -> Result<(), Error> {
        self.gatt.unregister().await
    }

    async fn start_advertising(&self, name: &str, uuids: &[Uuid]) -> Result<(), Error> {
        self.advertisement.add_name(name);
        self.advertisement.add_uuids(
            uuids
                .to_vec()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        );

        self.advertisement.register().await
    }

    async fn stop_advertising(&self) -> Result<(), Error> {
        self.advertisement.unregister().await
    }

    async fn is_advertising(&self) -> Result<bool, Error> {
        Ok(self.advertisement.is_advertising())
    }

    async fn add_service(&self, service: &Service) -> Result<(), Error> {
        self.gatt.add_service(service).await
    }
}
