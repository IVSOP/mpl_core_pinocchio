use crate::{
    data::{plugins::PluginAuthorityPair, Serialize},
    Instructions,
};

pub struct CreateCollectionV1InstructionData<'a> {
    pub name: &'a [u8],
    pub uri: &'a [u8],
    pub plugins: Option<&'a [PluginAuthorityPair<'a>]>,
}

impl<'a> Serialize for CreateCollectionV1InstructionData<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = Instructions::CreateCollection.to_u8();
        let mut offset = 1;

        offset += self.name.serialize_to(&mut buffer[offset..]);
        offset += self.uri.serialize_to(&mut buffer[offset..]);
        offset += self.plugins.serialize_to(&mut buffer[offset..]);

        return offset;
    }
}
