#![no_std]

use pinocchio::pubkey::Pubkey;
use pinocchio_pubkey::pubkey;

pub mod data;
pub mod instructions;

/// For internal use, to get the discriminant of the instruction
#[repr(u8)]
pub(crate) enum Instructions {
    CreateAsset = 0,
    CreateCollection = 1,
    UpdateAssetPlugin = 6,
    UpdateCollectionPlugin = 7,
    BurnAsset = 12,
    BurnCollection = 13,
    TransferAsset = 14,
}

pub const MPL_CORE_ID: Pubkey = pubkey!("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
