//! In-memory [`UsbBackend`] for hardware-free testing.
//!
//! [`MockBackend`] replays scripted IN responses per endpoint and records all
//! OUT writes so tests can assert on the exact bytes sent to the "device".
//! This lets the whole protocol stack ([`crate::msc`], future PTP/MTP code) be
//! exercised without a physical camera or any USB system library.

use std::collections::VecDeque;

use crate::{Error, Result, UsbBackend};

/// A recorded OUT transfer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteRecord {
    /// Endpoint address the data was written to.
    pub ep: u8,
    /// The bytes written.
    pub data: Vec<u8>,
}

/// A recorded vendor control OUT transfer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VendorRequestRecord {
    /// `bRequest` value.
    pub request: u8,
    /// `wValue` field.
    pub value: u16,
    /// `wIndex` field.
    pub index: u16,
    /// Payload of the data stage.
    pub data: Vec<u8>,
}

/// Scripted, in-memory USB backend.
#[derive(Default)]
pub struct MockBackend {
    reads: VecDeque<(u8, Vec<u8>)>,
    /// All OUT writes, in order.
    pub writes: Vec<WriteRecord>,
    /// All vendor control OUT transfers, in order.
    pub vendor_requests: Vec<VendorRequestRecord>,
    /// Number of times [`UsbBackend::reset`] was called.
    pub resets: usize,
}

impl MockBackend {
    /// Creates an empty mock with no scripted responses.
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues bytes to be returned by the next `read` on endpoint `ep`.
    pub fn queue_read(&mut self, ep: u8, data: Vec<u8>) {
        self.reads.push_back((ep, data));
    }
}

impl UsbBackend for MockBackend {
    fn read(&mut self, ep: u8, length: usize) -> Result<Vec<u8>> {
        match self.reads.pop_front() {
            Some((queued_ep, data)) if queued_ep == ep => {
                // Never return more than the caller asked for.
                Ok(data.into_iter().take(length).collect())
            }
            Some((queued_ep, _)) => Err(Error::Transfer(format!(
                "mock read on ep {ep:#x} but next scripted response targets ep {queued_ep:#x}"
            ))),
            None => {
                Err(Error::Transfer(format!("mock read on ep {ep:#x} with no scripted response")))
            }
        }
    }

    fn write(&mut self, ep: u8, data: &[u8]) -> Result<()> {
        self.writes.push(WriteRecord { ep, data: data.to_vec() });
        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        self.resets += 1;
        Ok(())
    }

    fn vendor_request_out(
        &mut self,
        request: u8,
        value: u16,
        index: u16,
        data: &[u8],
    ) -> Result<()> {
        self.vendor_requests.push(VendorRequestRecord {
            request,
            value,
            index,
            data: data.to_vec(),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_writes_and_replays_reads() {
        let mut b = MockBackend::new();
        b.queue_read(0x81, vec![1, 2, 3, 4]);
        b.write(0x02, &[0xaa, 0xbb]).unwrap();

        assert_eq!(b.read(0x81, 3).unwrap(), vec![1, 2, 3]);
        assert_eq!(b.writes[0], WriteRecord { ep: 0x02, data: vec![0xaa, 0xbb] });
    }

    #[test]
    fn read_on_wrong_endpoint_errors() {
        let mut b = MockBackend::new();
        b.queue_read(0x81, vec![0]);
        assert!(b.read(0x83, 1).is_err());
    }
}
