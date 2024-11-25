use crate::State;
use std::path::Path;

pub struct Sled {
    db: sled::Db,
}

#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error(transparent)]
    Sled(#[from] std::io::Error),
    #[error("Height mismatch: expected {expected}, found {found}")]
    HeightMismatch { expected: u64, found: u64 },
    #[error("Disallowed value")]
    DisallowedValue,
}

pub enum DbChange {
    Insert { key: Vec<u8>, value: Vec<u8> },
    Delete { key: Vec<u8> },
}

impl Sled {
    pub const LATEST_HEIGHT_KEY: &'static [u8] = b"__latest_height__";

    pub fn new(path: impl AsRef<Path>) -> Result<Sled, DbError> {
        let db = sled::open(path)?;
        Ok(Sled { db })
    }

    pub fn load_latest_state(&self) -> Result<ViewableState, DbError> {
        let latest_height = self
            .db
            .get(Self::LATEST_HEIGHT_KEY)?
            .map(|bytes| u64::from_be_bytes(bytes.as_ref().try_into().unwrap()))
            .unwrap_or(0);

        Ok(ViewableState::new(latest_height, self.db.clone()))
    }

    pub fn load_state_at(&self, height: u64) -> Result<ViewableState, DbError> {
        Ok(ViewableState::new(height, self.db.clone()))
    }

    pub fn commit_changes(
        &mut self,
        next_expected_height: u64,
        changes: Vec<DbChange>,
    ) -> Result<(), DbError> {
        let mut batch = sled::Batch::default();

        // Retrieve the current latest height
        let latest_height = self
            .db
            .get(Self::LATEST_HEIGHT_KEY)?
            .map(|bytes| u64::from_be_bytes(bytes.as_ref().try_into().unwrap()));

        // Ensure heights are sequential
        match latest_height {
            Some(height) => {
                if next_expected_height != height + 1 {
                    return Err(DbError::HeightMismatch {
                        expected: height + 1,
                        found: next_expected_height,
                    });
                }
            }
            None => {
                if next_expected_height != 0 {
                    return Err(DbError::HeightMismatch {
                        expected: 0,
                        found: next_expected_height,
                    });
                }
            }
        }

        // Invert the height for ordering (latest first)
        let inverted_height = u64::MAX - next_expected_height;
        let height_bytes = inverted_height.to_be_bytes();

        for change in changes {
            match change {
                DbChange::Insert { key, value } => {
                    let operation_flag = [0x00]; // 0x00 for insertion
                    let composite_key = [key.as_slice(), &height_bytes, &operation_flag].concat();
                    batch.insert(composite_key, value);
                }
                DbChange::Delete { key } => {
                    let operation_flag = [0x01]; // 0x01 for deletion
                    let composite_key = [key.as_slice(), &height_bytes, &operation_flag].concat();
                    batch.insert(composite_key, Vec::new()); // Value can be empty
                }
            }
        }

        // Update the latest height
        batch.insert(Self::LATEST_HEIGHT_KEY, &next_expected_height.to_be_bytes());

        // Apply the batch atomically
        self.db.apply_batch(batch)?;

        Ok(())
    }
}

pub struct ViewableState {
    pub height: u64,
    db: sled::Db,
}

impl ViewableState {
    pub fn new(height: u64, db: sled::Db) -> ViewableState {
        ViewableState { height, db }
    }
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, DbError> {
        // Compute the inverted height for the desired version
        let desired_inverted_height = u64::MAX - self.height;
        let desired_height_bytes = desired_inverted_height.to_be_bytes();

        // Construct the start key: [key][desired_inverted_height][0x00]
        let mut start_key = key.to_vec();
        start_key.extend_from_slice(&desired_height_bytes);
        start_key.push(0x00); // Minimum operation flag

        // Construct the end key: [key][0xFF...FF][0xFF]
        let mut end_key = key.to_vec();
        end_key.extend_from_slice(&[0xFF; 8]);
        end_key.push(0xFF); // Maximum operation flag

        // Create the range
        let range = start_key..=end_key;

        // Iterate over the range
        let iter = self.db.range(range);

        // The first key in the range is the one we need
        for result in iter {
            let (k, v) = result?;
            // Verify that the key matches the original key prefix
            if k.starts_with(key) {
                let key_len = key.len();
                let height_bytes = &k[key_len..key_len + 8];
                let operation_flag = k[key_len + 8];

                // Ensure the height_bytes are exactly 8 bytes
                if height_bytes.len() != 8 {
                    panic!("Invalid entry in DB, possible corruption");
                }

                let inverted_height_k = u64::from_be_bytes(height_bytes.try_into().unwrap());
                let version_height = u64::MAX - inverted_height_k;

                // Check if the version height is at or before the desired height
                if version_height <= self.height {
                    if operation_flag == 0x00 {
                        // Insert operation
                        return Ok(Some(v.to_vec()));
                    } else if operation_flag == 0x01 {
                        // Delete operation
                        return Ok(None);
                    }
                }
            } else {
                // We've moved past the keys for this original key
                break;
            }
        }

        // Key not found at or before the desired height
        Ok(None)
    }
}

impl State for ViewableState {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, String> {
        self.get(key).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all() {
        let _ = std::fs::remove_dir_all("./cookies"); // Clean up previous test data
        let mut sled = Sled::new("./cookies").unwrap();
        sled.commit_changes(
            0,
            vec![
                DbChange::Insert {
                    key: b"abc".to_vec(),
                    value: b"def".to_vec(),
                },
                DbChange::Insert {
                    key: b"123".to_vec(),
                    value: b"ghi".to_vec(),
                },
            ],
        )
        .unwrap();

        sled.commit_changes(
            1,
            vec![DbChange::Delete {
                key: b"123".to_vec(),
            }],
        )
        .unwrap();

        let view = sled.load_state_at(0).unwrap();
        assert_eq!(view.get(b"abc").unwrap(), Some(b"def".to_vec()));
        assert_eq!(view.get(b"123").unwrap(), Some(b"ghi".to_vec()));

        let view = sled.load_state_at(1).unwrap();
        assert_eq!(view.get(b"123").unwrap(), None);
        assert_eq!(view.get(b"abc").unwrap(), Some(b"def".to_vec()));
    }
}
