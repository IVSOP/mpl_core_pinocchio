use crate::{
    data::{plugins::PluginAuthorityPair, Serialize},
    Instructions,
};

#[repr(u8)]
pub enum DataState {
    AccountState,
    LedgerState,
}

impl Serialize for DataState {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = match self {
            Self::AccountState => 0,
            Self::LedgerState => 1,
        };
        return 1;
    }
}

pub struct CreateAssetV1InstructionData<'a> {
    pub data_state: DataState,
    pub name: &'a [u8],
    pub uri: &'a [u8],
    pub plugins: Option<&'a [PluginAuthorityPair<'a>]>,
}

impl<'a> Serialize for CreateAssetV1InstructionData<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = Instructions::CreateAsset.to_u8();
        let mut offset = 1;

        offset += self.data_state.serialize_to(&mut buffer[offset..]);
        offset += self.name.serialize_to(&mut buffer[offset..]);
        offset += self.uri.serialize_to(&mut buffer[offset..]);
        offset += self.plugins.serialize_to(&mut buffer[offset..]);

        return offset;
    }
}
