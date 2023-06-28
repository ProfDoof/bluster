use futures::channel::oneshot;
use futures::SinkExt;
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::BitOr;
use std::sync::Arc;
use uuid::Uuid;
use windows::core::{ComInterface, Error, Interface, Type, GUID, HRESULT, HSTRING};
use windows::Devices::Bluetooth::{BluetoothAdapter, BluetoothError};

use crate::gatt::characteristic::{Characteristic, Secure, Write};
use crate::gatt::event::{Event, ReadRequest};
use crate::gatt::service::Service;
use crate::peripheral::winrt::guid::WinUuid;
use windows::Devices::Bluetooth::GenericAttributeProfile::{
    GattCharacteristicProperties, GattLocalCharacteristic, GattLocalCharacteristicParameters,
    GattProtectionLevel, GattReadRequestedEventArgs, GattServiceProvider,
    GattWriteRequestedEventArgs,
};
use windows::Foundation::{EventRegistrationToken, TypedEventHandler};
use windows::Storage::Streams::{Buffer, DataWriter, InMemoryRandomAccessStream};
use windows::{h, w};

pub(crate) struct PeripheralManager {
    services: HashMap<Uuid, GattServiceProvider>,
    characteristic_read_handlers: HashMap<
        (Uuid, Uuid),
        Arc<TypedEventHandler<GattLocalCharacteristic, GattReadRequestedEventArgs>>,
    >,
    characteristic_write_handlers: HashMap<
        (Uuid, Uuid),
        Arc<TypedEventHandler<GattLocalCharacteristic, GattWriteRequestedEventArgs>>,
    >,
}

struct WriteProtectionLevel(GattProtectionLevel);
struct ReadProtectionLevel(GattProtectionLevel);

impl PeripheralManager {
    pub(crate) fn new() -> Self {
        Self {
            services: HashMap::new(),
            characteristic_read_handlers: HashMap::new(),
            characteristic_write_handlers: HashMap::new(),
        }
    }

    pub(crate) async fn is_powered(&self) -> windows::core::Result<bool> {
        let adapter = BluetoothAdapter::GetDefaultAsync()?.await?;
        let radio = adapter.GetRadioAsync()?.await?;
        radio.State().map(|state| state.0 == 1)
    }

    pub(crate) async fn add_service(&mut self, service: &Service) -> windows::core::Result<()> {
        let uuid = WinUuid(service.uuid);
        let service_provider_result = GattServiceProvider::CreateAsync(uuid.into())?.await?;
        if service_provider_result.Error()? != BluetoothError::Success {
            return Err(Error::new(
                HRESULT(1),
                HSTRING::from("Error getting GattServiceProvider"),
            ));
        }
        self.services
            .insert(service.uuid, service_provider_result.ServiceProvider()?);

        let service_provider = self.services.get(&service.uuid).unwrap();

        for characteristic in &service.characteristics {
            let uuid = WinUuid(characteristic.uuid);
            let parameters: GattLocalCharacteristicParameters =
                GattLocalCharacteristicParameters::new()?;
            let (properties, write_protection_level, read_protection_level) =
                Self::extract_props_and_perms(characteristic)?;
            parameters.SetCharacteristicProperties(properties)?;
            if let Some(wpl) = write_protection_level {
                parameters.SetWriteProtectionLevel(wpl.0)?;
            }

            if let Some(rpl) = read_protection_level {
                parameters.SetReadProtectionLevel(rpl.0)?;
            }

            if let Some(value) = &characteristic.value {
                let stream = InMemoryRandomAccessStream::new()?;
                let data_writer = DataWriter::CreateDataWriter(&stream)?;
                data_writer.WriteBytes(value)?;
                parameters.SetStaticValue(&data_writer.DetachBuffer()?)?;
            }

            let characteristic_result = service_provider
                .Service()?
                .CreateCharacteristicAsync(uuid.into(), &parameters)?
                .await?;

            if characteristic_result.Error()? != BluetoothError::Success {
                return Err(Error::new(
                    HRESULT(1),
                    HSTRING::from("Error creating a characteristic"),
                ));
            }

            let win_characteristic = characteristic_result.Characteristic()?;

            if let Some(write) = &characteristic.properties.write {
                self.characteristic_write_handlers.insert(
                    (service.uuid, characteristic.uuid),
                    TypedEventHandler::new(
                        move |originator: &Option<GattLocalCharacteristic>,
                              args: &Option<GattWriteRequestedEventArgs>| {
                            let event_args = args.unwrap();
                            let characteristic = originator.unwrap();
                            let mut sender = write.sender().clone();
                            let (responder, receiver) = oneshot::channel();
                            if !sender.is_closed() {
                                let request = tokio::runtime::Handle::current().block_on(event_args.GetRequestAsync()?)?
                                let offset = request.Offset()?;
                                let mtu = event_args.Session()?.MaxPduSize()?;
                                tokio::runtime::Handle::current()
                                    .block_on(sender.send(Event::ReadRequest(ReadRequest {
                                        offset: offset.try_into()?,
                                        response: responder,
                                        mtu,
                                    })))
                                    .into()?;
                            }
                            Ok(())
                        },
                    ),
                );
            }

            win_characteristic.ReadRequested();
        }

        Ok(())
    }

    fn extract_props_and_perms(
        characteristic: &Characteristic,
    ) -> windows::core::Result<(
        GattCharacteristicProperties,
        Option<WriteProtectionLevel>,
        Option<ReadProtectionLevel>,
    )> {
        let mut properties = GattCharacteristicProperties::None;
        let write_protection_level = if let Some(write) = &characteristic.properties.write {
            let mut protection_level = None;
            properties |= match write {
                Write::WithResponse(secure) => {
                    protection_level = match secure {
                        Secure::Insecure(_) => Some(GattProtectionLevel::Plain),
                        Secure::Secure(_) => Some(GattProtectionLevel::EncryptionRequired),
                    };
                    GattCharacteristicProperties::Write
                }
                Write::WithoutResponse(_) => GattCharacteristicProperties::WriteWithoutResponse,
            };
            protection_level
        } else {
            None
        }
        .map(WriteProtectionLevel);

        let read_protection_level = if let Some(read) = &characteristic.properties.read {
            properties |= GattCharacteristicProperties::Read;
            match read.0 {
                Secure::Secure(_) => Some(GattProtectionLevel::EncryptionRequired),
                Secure::Insecure(_) => Some(GattProtectionLevel::Plain),
            }
        } else {
            None
        }
        .map(ReadProtectionLevel);

        Ok((properties, write_protection_level, read_protection_level))
    }
}
