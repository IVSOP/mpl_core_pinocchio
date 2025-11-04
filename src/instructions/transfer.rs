use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

use crate::data::{transfer::TransferV1InstructionData, Serialize};

/// Transfer an asset
///
/// ### Accounts:
///   0. `[WRITE]` Asset
///   1. `[OPTIONAL]` Collection
///   2. `[WRITE, SIGNER]` Payer
///   3. `[SIGNER, OPTIONAL]` Authority
///   4. `[]` New owner
///   5. `[]` System Program
///   6. `[OPTIONAL]` SPL Noop
///   7. `[]` Metaplex Core Program
///
/// Accounts being optional is very cursed but mimics the behaviour of the official lib.
/// Accounts set to None get replaced by mpl core program's account.
/// Even the owner, which says "Defaults to the authority if not present", will get replaced inside the actual MPL program
pub struct TransferV1<'a> {
    /// The asset to transfer
    pub asset: &'a AccountInfo,
    /// The collection the asset belongs to
    pub collection: Option<&'a AccountInfo>,
    /// Payer
    pub payer: &'a AccountInfo,
    /// The authority
    pub authority: Option<&'a AccountInfo>,
    /// New owner of the asset
    pub new_owner: &'a AccountInfo,
    /// The system program
    pub system_program: &'a AccountInfo,
    /// The SPL Noop Program
    pub log_wrapper: Option<&'a AccountInfo>,
    /// The Metaplex Core Program
    pub mpl_core: &'a AccountInfo,
    // FIX: I have never ever had to use remaining accounts
    // I did not have the will figure out how to concatenate them into the other accounts
    // if you need this then do it
    // pub remaining_accounts: &'a [&'a AccountInfo]
}

impl TransferV1<'_> {
    #[inline(always)]
    pub fn invoke(
        &self,
        data: &TransferV1InstructionData,
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        self.invoke_signed(data, &[], serialization_buffer)
    }

    pub fn invoke_signed(
        &self,
        data: &TransferV1InstructionData,
        signers: &[Signer],
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        // account metadata
        let account_metas: &[AccountMeta] = &[
            AccountMeta::writable(self.asset.key()),
            match self.collection {
                Some(collection) => AccountMeta::readonly(collection.key()),
                None => AccountMeta::readonly(self.mpl_core.key()),
            },
            AccountMeta::writable_signer(&self.payer.key()),
            match self.authority {
                Some(authority) => AccountMeta::readonly_signer(authority.key()),
                None => AccountMeta::readonly(self.mpl_core.key()),
            },
            AccountMeta::readonly(self.new_owner.key()),
            AccountMeta::readonly(self.system_program.key()),
            match self.log_wrapper {
                Some(log_wrapper) => AccountMeta::readonly(log_wrapper.key()),
                None => AccountMeta::readonly(self.mpl_core.key()),
            },
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
                self.asset,
                self.collection.unwrap_or(self.mpl_core),
                self.payer,
                self.authority.unwrap_or(self.mpl_core),
                self.new_owner,
                self.system_program,
                self.log_wrapper.unwrap_or(self.mpl_core),
            ],
            signers,
        )
    }
}
