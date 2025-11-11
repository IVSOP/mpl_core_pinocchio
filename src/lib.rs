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

impl From<u8> for Instructions {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::CreateAsset,
            1 => Self::CreateCollection,
            6 => Self::UpdateAssetPlugin,
            7 => Self::UpdateCollectionPlugin,
            12 => Self::BurnAsset,
            13 => Self::BurnCollection,
            14 => Self::TransferAsset,
            _ => panic!("Invalid instruction value: {}", value),
        }
    }
}

impl Instructions {
    pub fn to_u8(self) -> u8 {
        match self {
            Self::CreateAsset => 0,
            Self::CreateCollection => 1,
            Self::UpdateAssetPlugin => 6,
            Self::UpdateCollectionPlugin => 7,
            Self::BurnAsset => 12,
            Self::BurnCollection => 13,
            Self::TransferAsset => 14,
        }
    }
}

pub const MPL_CORE_ID: Pubkey = pubkey!("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
