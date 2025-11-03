use crate::{
    data::{plugins::CompressionProof, Serialize},
    Instructions,
};

pub struct BurnCollectionV1InstructionData<'a> {
    pub compression_proof: Option<CompressionProof<'a>>,
}

impl<'a> Serialize for BurnCollectionV1InstructionData<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = Instructions::BurnCollection as u8;
        let mut offset = 1;

        offset += self.compression_proof.serialize_to(&mut buffer[offset..]);

        return offset;
    }
}
