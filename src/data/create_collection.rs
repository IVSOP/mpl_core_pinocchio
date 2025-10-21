use crate::{
    data::{plugins::PluginAuthorityPair, Serialize},
    Instructions,
};

pub struct CreateCollectionInstructionData<'a> {
    pub name: &'a str,
    pub uri: &'a str,
    pub plugins: Option<&'a [PluginAuthorityPair<'a>]>,
}

impl<'a> Serialize for CreateCollectionInstructionData<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = Instructions::CreateCollection as u8;
        let mut offset = 1;

        offset += self.name.serialize_to(&mut buffer[offset..]);
        offset += self.uri.serialize_to(&mut buffer[offset..]);
        offset += self.plugins.serialize_to(&mut buffer[offset..]);

        return offset;
    }
}
