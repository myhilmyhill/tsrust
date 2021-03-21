#[cfg(test)]
mod tests;

use std::fmt::{Debug, Formatter, Result};

pub struct RawBytes<T: IntoIterator<Item = u8>>(pub T);

impl<T: IntoIterator<Item = u8> + Clone> Debug for RawBytes<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for b in self.0.clone() {
            write!(f, "{:02x} ", b)?;
        }
        write!(f, "\x08") // backspace
    }
}
