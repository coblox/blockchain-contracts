use crate::ethereum::{Address, TokenQuantity};
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use rust_bitcoin::hashes::hash160;

pub trait FitIntoPlaceholderSlice {
    fn fit_into_placeholder_slice(self, buf: &mut [u8]);
}

impl FitIntoPlaceholderSlice for Address {
    fn fit_into_placeholder_slice(self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.0[..]);
    }
}

impl FitIntoPlaceholderSlice for TokenQuantity {
    fn fit_into_placeholder_slice(self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.0[..]);
    }
}

impl FitIntoPlaceholderSlice for EthereumTimestamp {
    fn fit_into_placeholder_slice(self, buf: &mut [u8]) {
        BigEndian::write_u32(buf, self.0);
    }
}

impl FitIntoPlaceholderSlice for BitcoinTimestamp {
    fn fit_into_placeholder_slice(self, buf: &mut [u8]) {
        LittleEndian::write_u32(buf, self.0);
    }
}

impl FitIntoPlaceholderSlice for SecretHash {
    fn fit_into_placeholder_slice(self, buf: &mut [u8]) {
        buf.copy_from_slice(&self.0[..]);
    }
}

impl FitIntoPlaceholderSlice for hash160::Hash {
    fn fit_into_placeholder_slice(self, buf: &mut [u8]) {
        buf.copy_from_slice(&self[..]);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SecretHash(pub [u8; 32]);

#[derive(Clone, Copy, Debug)]
pub struct EthereumTimestamp(pub u32);

#[derive(Debug)]
pub struct BitcoinTimestamp(pub u32);
