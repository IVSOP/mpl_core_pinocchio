use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

use crate::{
    data::{create_asset::CreateAssetInstructionData, Serialize},
    MAX_DATA_LEN,
};

/// Create an asset
///
/// ### Accounts:
///   0. `[WRITE, SIGNER]` Asset
///   1. `[WRITE]` Collection
///   2. `[SIGNER]` Authority
///   3. `[WRITE, SIGNER]` Payer
///   4. `[]` Owner
///   5. `[]` Update Authority
///   6. `[]` System Program
///   7. `[]` SPL Noop
///   8. `[]` Metaplex Core Program
///
/// Accounts being optional is very cursed but mimics the behaviour of the official lib.
/// Accounts set to None get replaced by mpl core program's account.
/// Even the owner, which says "Defaults to the authority if not present", will get replaced inside the actual MPL program
pub struct CreateAsset<'a> {
    /// The address of the new asset
    pub asset: &'a AccountInfo,
    /// The collection to which the asset belongs
    pub collection: Option<&'a AccountInfo>,
    /// The authority signing for creation
    pub authority: Option<&'a AccountInfo>,
    /// The account paying for the storage fees
    pub payer: &'a AccountInfo,
    /// The owner of the new asset. Defaults to the authority if not present.
    pub owner: Option<&'a AccountInfo>,
    /// The authority on the new asset
    pub update_authority: Option<&'a AccountInfo>,
    /// The system program
    pub system_program: &'a AccountInfo,
    /// The SPL Noop Program
    pub log_wrapper: &'a AccountInfo,
    /// The Metaplex Core Program
    pub mpl_core: &'a AccountInfo,
    // FIX: I have never ever had to use remaining accounts
    // I did not have the will figure out how to concatenate them into the other accounts
    // if you need this then do it
    // pub remaining_accounts: &'a [&'a AccountInfo]
}

impl CreateAsset<'_> {
    #[inline(always)]
    pub fn invoke(&self, data: &CreateAssetInstructionData) -> ProgramResult {
        self.invoke_signed(data, &[])
    }

    #[inline(always)]
    pub fn invoke_signed(
        &self,
        data: &CreateAssetInstructionData,
        signers: &[Signer],
    ) -> ProgramResult {
        // account metadata
        let account_metas: &[AccountMeta] = &[
            AccountMeta::writable_signer(self.asset.key()),
            match self.collection {
                Some(collection) => AccountMeta::writable(collection.key()),
                None => AccountMeta::readonly(self.mpl_core.key()),
            },
            match self.authority {
                Some(authority) => AccountMeta::readonly_signer(authority.key()),
                None => AccountMeta::readonly(self.mpl_core.key()),
            },
            AccountMeta::writable_signer(self.payer.key()),
            match self.owner {
                Some(owner) => AccountMeta::readonly(owner.key()),
                None => AccountMeta::readonly(self.mpl_core.key()),
            },
            match self.update_authority {
                Some(update_authority) => AccountMeta::readonly(update_authority.key()),
                None => AccountMeta::readonly(self.mpl_core.key()),
            },
            AccountMeta::readonly(self.system_program.key()),
            AccountMeta::readonly(self.log_wrapper.key()),
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
                self.asset,
                self.collection.unwrap_or(self.mpl_core),
                self.authority.unwrap_or(self.mpl_core),
                self.payer,
                self.owner.unwrap_or(self.mpl_core),
                self.update_authority.unwrap_or(self.mpl_core),
                self.system_program,
                self.log_wrapper,
            ],
            signers,
        )
    }
}
