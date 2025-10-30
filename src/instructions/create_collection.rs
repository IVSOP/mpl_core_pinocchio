use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

use crate::data::{create_collection::CreateCollectionV1InstructionData, Serialize};

/// Create a collection
///
/// ### Accounts:
///   0. `[WRITE, SIGNER]` Collection
///   1. `[OPTIONAL]` Update Authority
///   2. `[WRITE, SIGNER]` Payer
///   3. `[]` System Program
///   4. `[]` Metaplex Core Program
///
/// Accounts being optional is very cursed but mimics the behaviour of the official lib.
/// Accounts set to None get replaced by mpl core program's account.
/// FIX: why do they call collections assets wtfffff
pub struct CreateCollectionV1<'a> {
    /// The address of the new asset
    pub collection: &'a AccountInfo,
    /// The authority of the new asset
    pub update_authority: Option<&'a AccountInfo>,
    /// The account paying for the storage fees
    pub payer: &'a AccountInfo,
    /// The system program
    pub system_program: &'a AccountInfo,
    /// The Metaplex Core Program
    pub mpl_core: &'a AccountInfo,
}

impl CreateCollectionV1<'_> {
    #[inline(always)]
    pub fn invoke(
        &self,
        data: &CreateCollectionV1InstructionData,
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        self.invoke_signed(data, &[], serialization_buffer)
    }

    pub fn invoke_signed(
        &self,
        data: &CreateCollectionV1InstructionData,
        signers: &[Signer],
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        // account metadata
        let account_metas: &[AccountMeta] = &[
            AccountMeta::writable_signer(self.collection.key()),
            match self.update_authority {
                Some(update_authority) => AccountMeta::readonly(update_authority.key()),
                None => AccountMeta::readonly(self.mpl_core.key()),
            },
            AccountMeta::writable_signer(self.payer.key()),
            AccountMeta::readonly(self.system_program.key()),
        ];

        let len = data.serialize_to(serialization_buffer);
        let data = &serialization_buffer[..len];

        let instruction = Instruction {
            program_id: &crate::MPL_CORE_ID,
            accounts: &account_metas,
            data,
        };

        invoke_signed(
            &instruction,
            &[
                self.collection,
                self.update_authority.unwrap_or(self.mpl_core),
                self.payer,
                self.system_program,
            ],
            signers,
        )
    }
}
