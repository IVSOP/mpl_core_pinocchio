use crate::{
    data::{plugins::CompressionProof, Serialize},
    Instructions,
};

pub struct TransferV1InstructionData<'a> {
    pub compression_proof: Option<CompressionProof<'a>>
}

impl<'a> Serialize for TransferV1InstructionData <'a>{
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        buffer[0] = Instructions::CreateCollection as u8;
        let mut offset = 1;

        offset += self.compression_proof.serialize_to(&mut buffer[offset..]);

        return offset;
    }
}
