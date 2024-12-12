use crate::map::MAX_SIZE;
use ixc_core::resource::InitializationError;

pub(crate) struct Prefix {
    length: u8,
    data: [u8; 7],
}

impl Prefix {
    pub(crate) fn new(scope: &[u8], prefix: u8) -> Result<Self, InitializationError> {
        let length = scope.len() + 1;
        if length > MAX_SIZE {
            return Err(InitializationError::ExceedsLength);
        }
        let mut data: [u8; MAX_SIZE] = [0u8; MAX_SIZE];
        data[0..scope.len()].copy_from_slice(scope);
        data[scope.len()] = prefix;

        Ok(Prefix {
            length: length as u8,
            data,
        })
    }

    /// as_slice returns the underlying slice of the prefix.
    pub(crate) fn as_slice(&self) -> &[u8] {
        &self.data[..self.length as usize]
    }
}
