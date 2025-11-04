use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

pub mod asset;
pub mod burn_asset;
pub mod burn_collection;
pub mod create_asset;
pub mod create_collection;
pub mod plugins;
pub mod transfer;
pub mod update_asset_plugin;
pub mod update_collection_plugin;

pub trait Serialize {
    /// Serialize into a slice, starting at 0, returning how many bytes were written
    fn serialize_to(&self, buffer: &mut [u8]) -> usize;
}

pub trait DeserializeSized {
    fn deserialize(bytes: &[u8]) -> Result<Self, ProgramError>
    where
        Self: Sized;
}

pub trait Skip {
    fn skip_bytes(bytes: &[u8]) -> Result<usize, ProgramError>;
}

// faster but items must be sized
pub fn skip_sized<T: Sized>() -> usize {
    size_of::<T>()
}

// faster but items must be sized
pub fn skip_sized_slice<T: Sized>(bytes: &[u8]) -> Result<usize, ProgramError> {
    let len = u32::deserialize(bytes)?;
    Ok(4 + (size_of::<T>() * len as usize))
}

impl Serialize for &str {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let bytes = self.as_bytes();
        let len = bytes.len();
        let total_len = 4 + len;

        buffer[..4].copy_from_slice(&(len as u32).to_le_bytes());
        buffer[4..total_len].copy_from_slice(bytes);

        total_len
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        match self {
            None => {
                buffer[0] = 0;
                1
            }
            Some(data) => {
                buffer[0] = 1;
                1 + data.serialize_to(&mut buffer[1..])
            }
        }
    }
}

impl<T: Skip> Skip for Option<T> {
    fn skip_bytes(buffer: &[u8]) -> Result<usize, ProgramError> {
        let disc = buffer[0];
        match disc {
            0 => Ok(1),
            1 => Ok(1 + T::skip_bytes(&buffer[1..])?),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}

impl<T: Serialize> Serialize for [T] {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let len = self.len() as u32;
        buffer[..4].copy_from_slice(&len.to_le_bytes());

        let mut offset = 4;

        for item in self {
            offset += item.serialize_to(&mut buffer[offset..]);
        }

        offset
    }
}

impl<T: Serialize> Serialize for &[T] {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let len = self.len() as u32;
        buffer[..4].copy_from_slice(&len.to_le_bytes());

        let mut offset = 4;

        for item in self.iter() {
            offset += item.serialize_to(&mut buffer[offset..]);
        }

        offset
    }
}

impl<T: Skip> Skip for &[T] {
    fn skip_bytes(bytes: &[u8]) -> Result<usize, ProgramError> {
        let len = u32::from_le_bytes(
            bytes[0..4]
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        );

        let mut offset = 4;
        for _ in 0..len {
            offset += T::skip_bytes(&bytes[offset..])?;
        }

        Ok(offset)
    }
}

impl Serialize for Pubkey {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[..32].copy_from_slice(self);
        32
    }
}

impl Serialize for u8 {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = *self;
        1
    }
}

impl Serialize for u16 {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[..2].copy_from_slice(&self.to_le_bytes());
        2
    }
}

impl DeserializeSized for u16 {
    fn deserialize(bytes: &[u8]) -> Result<Self, ProgramError> {
        Ok(u16::from_le_bytes(
            bytes[0..2]
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        ))
    }
}

impl Serialize for u32 {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[..4].copy_from_slice(&self.to_le_bytes());
        4
    }
}

impl DeserializeSized for u32 {
    fn deserialize(bytes: &[u8]) -> Result<Self, ProgramError> {
        Ok(u32::from_le_bytes(
            bytes[0..4]
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        ))
    }
}

impl Serialize for u64 {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[..8].copy_from_slice(&self.to_le_bytes());
        8
    }
}

impl Skip for u64 {
    fn skip_bytes(_bytes: &[u8]) -> Result<usize, ProgramError> {
        Ok(8)
    }
}

impl DeserializeSized for u64 {
    fn deserialize(bytes: &[u8]) -> Result<Self, ProgramError> {
        Ok(u64::from_le_bytes(
            bytes[0..8]
                .try_into()
                .map_err(|_| ProgramError::InvalidAccountData)?,
        ))
    }
}

impl Serialize for bool {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = if *self { 1 } else { 0 };
        1
    }
}
