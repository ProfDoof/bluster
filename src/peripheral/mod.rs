#[cfg(any(target_os = "macos", target_os = "ios"))]
mod corebluetooth;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use self::corebluetooth::Peripheral;

#[cfg(target_os = "windows")]
mod winrt;
#[cfg(target_os = "windows")]
pub use self::winrt::Peripheral;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod bluez;

#[cfg(any(target_os = "linux", target_os = "android"))]
pub use self::bluez::Peripheral;

use crate::gatt::service::Service;
use crate::Error;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PeripheralServer {
    async fn is_powered(&self) -> Result<bool, Error>;
    async fn register_gatt(&self) -> Result<(), Error>;
    async fn unregister_gatt(&self) -> Result<(), Error>;
    async fn start_advertising(&self, name: &str, uuids: &[Uuid]) -> Result<(), Error>;
    async fn stop_advertising(&self) -> Result<(), Error>;
    async fn is_advertising(&self) -> Result<bool, Error>;
    async fn add_service(&self, service: &Service) -> Result<(), Error>;
}
// TODO: Add struct / traits to implement for each OS
//
// pub enum BindingsEvent {
//     StateChange,
//     Platform,
//     AddressChange,
//     AdvertisingStart,
//     AdvertisingStop,
//     ServicesSet,
//     Accept,
//     MtuChange,
//     Disconnect,
//     RssiUpdate,
// }
//
// #[derive(Debug, Clone)]
// pub enum State {
//     Unknown,
//     Resetting,
//     Unsupported,
//     Unauthorized,
//     PoweredOff,
//     PoweredOn,
// }
//
// #[derive(Debug, Clone)]
// pub struct Ble {
//     initialized: bool,
//     platform: String, // TODO: Make this an enum?
//     state: State,
//     address: String, // TODO: Make this a struct or something?
//     rssi: u8,
//     mtu: u8,
// }
