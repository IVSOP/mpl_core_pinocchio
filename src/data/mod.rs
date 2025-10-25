use pinocchio::pubkey::Pubkey;

pub mod create_asset;
pub mod create_collection;
pub mod plugins;
pub mod update_asset_plugin;
pub mod update_collection_plugin;
pub mod transfer;

pub trait Serialize {
    /// Serialize into a slice, starting at 0, returning how many bytes were written
    fn serialize_to(&self, buffer: &mut [u8]) -> usize;
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

impl Serialize for u32 {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[..4].copy_from_slice(&self.to_le_bytes());
        4
    }
}

impl Serialize for u64 {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[..8].copy_from_slice(&self.to_le_bytes());
        8
    }
}

impl Serialize for bool {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        if *self {
            buffer[0] = 0;
        } else {
            buffer[0] = 1;
        }

        1
    }
}
