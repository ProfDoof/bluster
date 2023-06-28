use uuid::Uuid;
use windows::core::GUID;

pub(crate) struct WinUuid(pub(crate) Uuid);

impl From<Uuid> for WinUuid {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<WinUuid> for GUID {
    fn from(value: WinUuid) -> Self {
        let (g1, g2, g3, g4) = value.0.as_fields();
        GUID::from_values(g1, g2, g3, g4.clone())
    }
}
