use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_signed,
    instruction::{AccountMeta, Instruction, Signer},
    ProgramResult,
};

use crate::data::{plugins::Plugin, Serialize};

/// Update a collection
///
/// ### Accounts:
///   0. `[WRITE]` Collection
///   1. `[WRITE, SIGNER]` payer
///   2. `[SIGNER]` Authority
///   4. `[]` System Program
///   5. `[]` SPL Noop
///   6. `[]` Metaplex Core Program
///
/// Accounts being optional is very cursed but mimics the behaviour of the official lib.
/// Accounts set to None get replaced by mpl core program's account.
/// Even the owner, which says "Defaults to the authority if not present", will get replaced inside the actual MPL program
pub struct UpdateCollectionPluginV1<'a> {
    /// The collection to update
    pub collection: &'a AccountInfo,
    /// The payer
    pub payer: &'a AccountInfo,
    /// The authority
    pub authority: Option<&'a AccountInfo>,
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

impl UpdateCollectionPluginV1<'_> {
    #[inline(always)]
    pub fn invoke(
        &self,
        plugin: &Plugin,
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        self.invoke_signed(plugin, &[], serialization_buffer)
    }

    #[inline(always)]
    pub fn invoke_signed(
        &self,
        plugin: &Plugin,
        signers: &[Signer],
        serialization_buffer: &mut [u8],
    ) -> ProgramResult {
        // account metadata
        let account_metas: &[AccountMeta] = &[
            AccountMeta::writable(self.collection.key()),
            AccountMeta::writable_signer(&self.payer.key()),
            match self.authority {
                Some(authority) => AccountMeta::readonly_signer(authority.key()),
                None => AccountMeta::readonly(self.mpl_core.key()),
            },
            AccountMeta::readonly(self.system_program.key()),
            match self.log_wrapper {
                Some(log_wrapper) => AccountMeta::readonly(log_wrapper.key()),
                None => AccountMeta::readonly(self.mpl_core.key()),
            },
        ];

        let len = plugin.serialize_to(serialization_buffer);
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
                self.payer,
                self.authority.unwrap_or(self.mpl_core),
                self.system_program,
                self.log_wrapper.unwrap_or(self.mpl_core),
            ],
            signers,
        )
    }
}
