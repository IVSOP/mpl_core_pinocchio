use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

use crate::{
    data::{create_asset::CreateAssetInstructionData, create_collection::CreateCollectionInstructionData, Serialize},
    MAX_DATA_LEN,
};

/// Create a collection
///
/// ### Accounts:
///   0. `[WRITE, SIGNER]` Collection
///   1. `[]` Update Authority
///   2. `[WRITE, SIGNER]` Payer
///   3. `[]` System Program
///   4. `[]` Metaplex Core Program
///
/// Accounts being optional is very cursed but mimics the behaviour of the official lib.
/// Accounts set to None get replaced by mpl core program's account.
/// FIX: why do they call collections assets wtfffff
pub struct CreateCollection<'a> {
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

impl CreateCollection<'_> {
    #[inline(always)]
    pub fn invoke(&self, data: &CreateCollectionInstructionData) -> ProgramResult {
        self.invoke_signed(data, &[])
    }

    #[inline(always)]
    pub fn invoke_signed(
        &self,
        data: &CreateCollectionInstructionData,
        signers: &[Signer],
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

        let mut instruction_data = [0_u8; MAX_DATA_LEN];
        data.serialize_to(&mut instruction_data);

        let instruction = Instruction {
            program_id: &crate::MPL_CORE_ID,
            accounts: &account_metas,
            data: &instruction_data,
        };

        invoke_signed(
            &instruction,
            &[
                self.collection,
                self.update_authority.unwrap_or(self.mpl_core),
                self.payer,
                self.update_authority.unwrap_or(self.mpl_core),
                self.system_program,
            ],
            signers,
        )
    }
}
