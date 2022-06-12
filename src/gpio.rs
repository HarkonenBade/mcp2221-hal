use hidapi::{HidDevice, HidResult};

pub struct GP0<'a> {
    pub(crate) dev: &'a HidDevice,
}