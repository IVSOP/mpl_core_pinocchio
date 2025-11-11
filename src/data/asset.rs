use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

use crate::data::{
    plugins::{Plugin, PluginAuthority, UpdateAuthority},
    skip_sized, skip_sized_slice, DeserializeSized, Serialize, Skip,
};
use core::mem::MaybeUninit;

/// This struct contains processed data about an asset.
/// TODO: for now external plugins are not supported
/// I made this different from how the data looks so that it is not terrible to use
pub struct AssetInfo<'a> {
    pub base: BaseAssetV1<'a>,
    pub plugins: &'a [PluginAuthorityPairWithoutOption<'a>],
}

/// I don't fucking know. They have one with an Option<PluginAuthority>, where the PluginAuthority itself can be None!!!!!!! literally None(None)!!!!!! who the fuck made this
pub struct PluginAuthorityPairWithoutOption<'a> {
    pub plugin: Plugin<'a>,
    pub authority: PluginAuthority,
}

impl<'a> Serialize for AssetInfo<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.base.serialize_to(buffer);

        // after the base asset, there is a PluginHeaderV1 pointing to where the registry is. This seems completely absolutely fucking monkey brained but whatever
        // I'll plop it down right here and assume there is no data between this and the asset
        // HOWEVER, this only applies if there are any plugins at all
        if self.plugins.is_empty() {
            return offset;
        }

        // there is a PluginHeaderV1, which we have to skip since it requires an offset we don't know yet....
        let plugin_header_offset = offset;
        offset += 9;

        // now we write all of the plugins
        // WARN: this is really ugly I don't care. while serializing I need to store how big each plugin was
        const MAX_PLUGINS_MAGIC_NUMBER: usize = 16;
        let mut registry_records: [MaybeUninit<RegistryRecordSafe>; MAX_PLUGINS_MAGIC_NUMBER] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for (registry_record, plugin_auth_pair) in
            registry_records.iter_mut().zip(self.plugins.iter())
        {
            let start_offset = offset;
            offset += plugin_auth_pair.plugin.serialize_to(&mut buffer[offset..]);

            *registry_record = MaybeUninit::new(RegistryRecordSafe {
                plugin_type: plugin_auth_pair.plugin.get_plugin_number(),
                authority: plugin_auth_pair.authority,
                offset: u64::try_from(start_offset).unwrap(), // didn't feel like changing this to be a Result<>
            });
        }

        let records_slice: &[RegistryRecordSafe] = unsafe {
            core::slice::from_raw_parts(
                registry_records.as_ptr() as *const RegistryRecordSafe,
                self.plugins.len(),
            )
        };

        let registry = PluginRegistryV1Safe {
            key: Key::PluginRegistryV1,
            registry: &records_slice,
            // TODO: for now external plugins are not supported
            external_registry: &[],
        };

        let registry_offset = offset;
        offset += registry.serialize_to(&mut buffer[offset..]);

        // now finally serialize the header.......
        let header = PluginHeaderV1 {
            key: Key::PluginHeaderV1,
            plugin_registry_offset: u64::try_from(registry_offset).unwrap(), // didn't feel like changing this to be a Result<>
        };

        header.serialize_to(&mut buffer[plugin_header_offset..]);

        offset
    }
}

pub struct BaseAssetV1<'a> {
    pub key: Key,
    pub owner: Pubkey,
    pub update_authority: UpdateAuthority,
    pub name: &'a [u8],
    pub uri: &'a [u8],
    pub seq: Option<u64>,
}

impl<'a> Serialize for BaseAssetV1<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.key.serialize_to(buffer);
        offset += self.owner.serialize_to(&mut buffer[offset..]);
        offset += self.update_authority.serialize_to(&mut buffer[offset..]);
        offset += self.name.serialize_to(&mut buffer[offset..]);
        offset += self.uri.serialize_to(&mut buffer[offset..]);
        offset += self.seq.serialize_to(&mut buffer[offset..]);
        offset
    }
}

impl<'a> Skip for BaseAssetV1<'_> {
    // DOES NOT ASSUME KEY WAS SKIPPED
    fn skip_bytes(bytes: &[u8]) -> Result<usize, ProgramError> {
        let mut offset: usize = 1;
        offset += skip_sized::<Pubkey>();
        offset += UpdateAuthority::skip_bytes(&bytes[offset..])?;
        offset += skip_sized_slice::<u8>(&bytes[offset..])?;
        offset += skip_sized_slice::<u8>(&bytes[offset..])?;
        offset += Option::<u64>::skip_bytes(&bytes[offset..])?;
        Ok(offset)
    }
}

pub struct BaseCollectionV1<'a> {
    pub key: Key,
    pub update_authority: Pubkey,
    pub name: &'a [u8],
    pub uri: &'a [u8],
    pub num_minted: u32,
    pub current_size: u32,
}

impl<'a> Skip for BaseCollectionV1<'_> {
    // DOES NOT ASSUME KEY WAS SKIPPED
    fn skip_bytes(bytes: &[u8]) -> Result<usize, ProgramError> {
        let mut offset: usize = 1;
        offset += skip_sized::<Pubkey>();
        offset += skip_sized_slice::<u8>(&bytes[offset..])?;
        offset += skip_sized_slice::<u8>(&bytes[offset..])?;
        offset += skip_sized::<u32>();
        offset += skip_sized::<u32>();
        Ok(offset)
    }
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Key {
    Uninitialized,
    AssetV1,
    HashedAssetV1,
    PluginHeaderV1,
    PluginRegistryV1,
    CollectionV1,
}

impl Serialize for Key {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = *self as u8;
        1
    }
}

impl Key {
    pub fn deserialize_from(bytes: &[u8]) -> Result<Self, ProgramError> {
        match bytes[0] {
            0 => Ok(Key::Uninitialized),
            1 => Ok(Key::AssetV1),
            2 => Ok(Key::HashedAssetV1),
            3 => Ok(Key::PluginHeaderV1),
            4 => Ok(Key::PluginRegistryV1),
            5 => Ok(Key::CollectionV1),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}

pub struct PluginHeaderV1 {
    pub key: Key,
    pub plugin_registry_offset: u64,
}

impl Serialize for PluginHeaderV1 {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.key.serialize_to(buffer);
        offset += self
            .plugin_registry_offset
            .serialize_to(&mut buffer[offset..]);
        offset
    }
}

impl DeserializeSized for PluginHeaderV1 {
    fn deserialize(bytes: &[u8]) -> Result<Self, ProgramError> {
        let key = Key::deserialize_from(bytes)?;

        if !matches!(key, Key::PluginHeaderV1) {
            return Err(ProgramError::InvalidAccountData);
        }
        let plugin_registry_offset = u64::deserialize(&bytes[1..])?;

        Ok(Self {
            key,
            plugin_registry_offset,
        })
    }
}

pub struct RegistryRecordSafe {
    pub plugin_type: u8,
    pub authority: PluginAuthority,
    pub offset: u64,
}

impl Serialize for RegistryRecordSafe {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.plugin_type.serialize_to(buffer);
        offset += self.authority.serialize_to(&mut buffer[offset..]);
        offset += self.offset.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct PluginRegistryV1Safe<'a> {
    pub key: Key,
    pub registry: &'a [RegistryRecordSafe],
    pub external_registry: &'a [ExternalRegistryRecordSafe<'a>],
}

impl<'a> Serialize for PluginRegistryV1Safe<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.key.serialize_to(buffer);
        offset += self.registry.serialize_to(&mut buffer[offset..]);
        offset += self.external_registry.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct ExternalCheckResult {
    pub flags: u32,
}

impl Serialize for ExternalCheckResult {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        self.flags.serialize_to(buffer)
    }
}

impl Serialize for (u8, ExternalCheckResult) {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.0.serialize_to(buffer);
        offset += self.1.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct ExternalRegistryRecordSafe<'a> {
    pub plugin_type: u8,
    pub authority: PluginAuthority,
    pub lifecycle_checks: Option<&'a [(u8, ExternalCheckResult)]>,
    pub offset: u64,
    pub data_offset: Option<u64>,
    pub data_len: Option<u64>,
}

impl<'a> Serialize for ExternalRegistryRecordSafe<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.plugin_type.serialize_to(buffer);
        offset += self.authority.serialize_to(&mut buffer[offset..]);
        offset += self.lifecycle_checks.serialize_to(&mut buffer[offset..]);
        offset += self.offset.serialize_to(&mut buffer[offset..]);
        offset += self.data_offset.serialize_to(&mut buffer[offset..]);
        offset += self.data_len.serialize_to(&mut buffer[offset..]);
        offset
    }
}
