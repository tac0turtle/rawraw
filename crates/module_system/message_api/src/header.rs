//! Message header structure.
use crate::account_id::AccountID;
use crate::data_pointer::DataPointer;

/// The size of a message header in bytes.
pub const MESSAGE_HEADER_SIZE: usize = size_of::<MessageHeader>();

/// A message header.
#[derive(Copy, Clone)]
#[non_exhaustive]
#[repr(C)]
pub struct MessageHeader {
    /// The target account of the message.
    pub account: AccountID, // 16 bytes
    /// The account sending the message.
    pub caller: AccountID, // 16 bytes
    /// If this is updated by the message handler itself, it indicates gas consumed
    /// in addition to anything tracked by the VM.
    pub gas_left: u64, // 8 bytes
    /// The message selector.
    pub message_selector: MessageSelector, // 8 bytes
    /// The amount of gas left.
    /// Input data pointer 1.
    pub in_pointer1: DataPointer, // 16 bytes
    /// Input data pointer 2.
    pub in_pointer2: DataPointer, // 16 bytes
    /// Output data pointer 1.
    pub out_pointer1: DataPointer, // 16 bytes
    /// Output data pointer 2.
    pub out_pointer2: DataPointer, // 16 bytes

    reserved: [u8; 16],
}

/// A message selector code.
pub type MessageSelector = u64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_header_size() {
        assert_eq!(MESSAGE_HEADER_SIZE, 128);
    }
}
