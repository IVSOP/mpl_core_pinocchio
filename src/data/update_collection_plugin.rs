use crate::{
    data::{plugins::Plugin, Serialize},
    Instructions,
};

pub struct UpdateCollectionPluginV1InstructionData<'a> {
    pub plugin: Plugin<'a>,
}

impl<'a> Serialize for UpdateCollectionPluginV1InstructionData<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = Instructions::UpdateCollectionPlugin as u8;
        let mut offset = 1;

        offset += self.plugin.serialize_to(&mut buffer[offset..]);

        return offset;
    }
}
