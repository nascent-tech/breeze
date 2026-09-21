use breeze_ports::{Display, DisplayEnumerationPort};

pub struct NullDisplays;

impl DisplayEnumerationPort for NullDisplays {
    fn displays(&self) -> Vec<Display> {
        Vec::new()
    }
}
