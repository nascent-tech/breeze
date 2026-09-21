use crate::display::Display;

pub trait DisplayEnumerationPort {
    fn displays(&self) -> Vec<Display>;
}
