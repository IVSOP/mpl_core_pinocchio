use pinocchio::pubkey::Pubkey;

use crate::data::Serialize;

pub struct Creator {
    pub address: Pubkey,
    pub percentage: u8,
}

impl Serialize for Creator {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.address.serialize_to(buffer);
        offset += self.percentage.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub enum RuleSet<'a> {
    None,
    ProgramAllowList(&'a [Pubkey]),
    ProgramDenyList(&'a [Pubkey]),
}

impl<'a> Serialize for RuleSet<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        match self {
            Self::None => {
                buffer[0] = 0;
                1
            }
            Self::ProgramAllowList(keys) => {
                buffer[0] = 1;
                1 + keys.serialize_to(&mut buffer[1..])
            }
            Self::ProgramDenyList(keys) => {
                buffer[0] = 2;
                1 + keys.serialize_to(&mut buffer[1..])
            }
        }
    }
}

pub struct Royalties<'a> {
    pub basis_points: u16,
    pub creators: &'a [Creator],
    pub rule_set: RuleSet<'a>,
}

impl<'a> Serialize for Royalties<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.basis_points.serialize_to(buffer);
        offset += self.creators.serialize_to(&mut buffer[offset..]);
        offset += self.rule_set.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct FreezeDelegate {
    pub frozen: bool,
}

impl Serialize for FreezeDelegate {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        self.frozen.serialize_to(buffer)
    }
}

pub struct PermanentFreezeDelegate {
    pub frozen: bool,
}

impl Serialize for PermanentFreezeDelegate {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        self.frozen.serialize_to(buffer)
    }
}

pub struct UpdateDelegate<'a> {
    pub additional_delegates: &'a [Pubkey],
}

impl<'a> Serialize for UpdateDelegate<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        self.additional_delegates.serialize_to(buffer)
    }
}

pub struct Attribute<'a> {
    pub key: &'a [u8],
    pub value: &'a [u8],
}

impl<'a> Serialize for Attribute<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.key.serialize_to(buffer);
        offset += self.value.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct Attributes<'a> {
    pub attribute_list: &'a [Attribute<'a>],
}

impl<'a> Serialize for Attributes<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        self.attribute_list.serialize_to(buffer)
    }
}

pub struct Edition {
    pub number: u32,
}

impl Serialize for Edition {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        self.number.serialize_to(buffer)
    }
}

pub struct MasterEdition<'a> {
    pub max_supply: Option<u32>,
    pub name: Option<&'a [u8]>,
    pub uri: Option<&'a [u8]>,
}

impl<'a> Serialize for MasterEdition<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.max_supply.serialize_to(buffer);
        offset += self.name.serialize_to(&mut buffer[offset..]);
        offset += self.uri.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct VerifiedCreatorsSignature {
    pub address: Pubkey,
    pub verified: bool,
}

impl Serialize for VerifiedCreatorsSignature {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.address.serialize_to(buffer);
        offset += self.verified.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct VerifiedCreators<'a> {
    pub signatures: &'a [VerifiedCreatorsSignature],
}

impl<'a> Serialize for VerifiedCreators<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        self.signatures.serialize_to(buffer)
    }
}

pub struct AutographSignature<'a> {
    pub address: Pubkey,
    pub message: &'a [u8],
}

impl<'a> Serialize for AutographSignature<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.address.serialize_to(buffer);
        offset += self.message.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct Autograph<'a> {
    pub signatures: &'a [AutographSignature<'a>],
}

impl<'a> Serialize for Autograph<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        self.signatures.serialize_to(buffer)
    }
}

pub struct FreezeExecute {
    pub frozen: bool,
}

impl Serialize for FreezeExecute {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        self.frozen.serialize_to(buffer)
    }
}

pub struct PermanentFreezeExecute {
    pub frozen: bool,
}

impl Serialize for PermanentFreezeExecute {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        self.frozen.serialize_to(buffer)
    }
}

// FIX: some plugins just had an empty struct inside so I left them empty here
// how are they even useful if they're just empty, wtf
pub enum Plugin<'a> {
    Royalties(Royalties<'a>),
    FreezeDelegate(FreezeDelegate),
    BurnDelegate,
    TransferDelegate,
    UpdateDelegate(UpdateDelegate<'a>),
    PermanentFreezeDelegate(PermanentFreezeDelegate),
    Attributes(Attributes<'a>),
    PermanentTransferDelegate,
    PermanentBurnDelegate,
    Edition(Edition),
    MasterEdition(MasterEdition<'a>),
    AddBlocker,
    ImmutableMetadata,
    VerifiedCreators(VerifiedCreators<'a>),
    Autograph(Autograph<'a>),
    BubblegumV2,
    FreezeExecute(FreezeExecute),
    PermanentFreezeExecute(PermanentFreezeExecute),
}

impl<'a> Serialize for Plugin<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        match self {
            Self::Royalties(royalties) => {
                buffer[0] = 0;
                1 + royalties.serialize_to(&mut buffer[1..])
            }
            Self::FreezeDelegate(freeze_delegate) => {
                buffer[0] = 1;
                1 + freeze_delegate.serialize_to(&mut buffer[1..])
            }
            Self::BurnDelegate => {
                buffer[0] = 2;
                1
            }
            Self::TransferDelegate => {
                buffer[0] = 3;
                1
            }
            Self::UpdateDelegate(update_delegate) => {
                buffer[0] = 4;
                1 + update_delegate.serialize_to(&mut buffer[1..])
            }
            Self::PermanentFreezeDelegate(permanent_freeze_delegate) => {
                buffer[0] = 5;
                1 + permanent_freeze_delegate.serialize_to(&mut buffer[1..])
            }
            Self::Attributes(attributes) => {
                buffer[0] = 6;
                1 + attributes.serialize_to(&mut buffer[1..])
            }
            Self::PermanentTransferDelegate => {
                buffer[0] = 7;
                1
            }
            Self::PermanentBurnDelegate => {
                buffer[0] = 7;
                1
            }
            Self::Edition(edition) => {
                buffer[0] = 8;
                1 + edition.serialize_to(&mut buffer[1..])
            }
            Self::MasterEdition(master_edition) => {
                buffer[0] = 9;
                1 + master_edition.serialize_to(&mut buffer[1..])
            }
            Self::AddBlocker => {
                buffer[0] = 10;
                1
            }
            Self::ImmutableMetadata => {
                buffer[0] = 11;
                1
            }
            Self::VerifiedCreators(verified_creators) => {
                buffer[0] = 12;
                1 + verified_creators.serialize_to(buffer)
            }
            Self::Autograph(autograph) => {
                buffer[0] = 13;
                1 + autograph.serialize_to(buffer)
            }
            Self::BubblegumV2 => {
                buffer[0] = 14;
                1
            }
            Self::FreezeExecute(freeze_execute) => {
                buffer[0] = 15;
                1 + freeze_execute.serialize_to(buffer)
            }
            Self::PermanentFreezeExecute(permanent_freeze_execute) => {
                buffer[0] = 16;
                1 + permanent_freeze_execute.serialize_to(buffer)
            }
        }
    }
}

#[repr(u8)]
pub enum PluginAuthority {
    None,
    Owner,
    UpdateAuthority,
    Address(Pubkey),
}

impl Serialize for PluginAuthority {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        match self {
            Self::None => {
                buffer[0] = 0;
                1
            }
            Self::Owner => {
                buffer[0] = 1;
                1
            }
            Self::UpdateAuthority => {
                buffer[0] = 2;
                1
            }
            Self::Address(key) => {
                buffer[0] = 3;
                1 + key.serialize_to(&mut buffer[1..])
            }
        }
    }
}

pub struct PluginAuthorityPair<'a> {
    pub plugin: Plugin<'a>,
    pub authority: Option<PluginAuthority>,
}

impl<'a> Serialize for PluginAuthorityPair<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.plugin.serialize_to(buffer);
        offset += self.authority.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub enum UpdateAuthority {
    None,
    Address(Pubkey),
    Collection(Pubkey),
}

impl Serialize for UpdateAuthority {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        match self {
            Self::None => {
                buffer[0] = 0;
                1
            },
            Self::Address(address) => {
                buffer[1] = 0;
                1 + address.serialize_to(&mut buffer[1..])
            },
            Self::Collection(collection) => {
                buffer[2] = 0;
                1 + collection.serialize_to(&mut buffer[1..])
            }
        }
    }
}

pub struct HashablePluginSchema<'a> {
    pub index: u64,
    pub authority: PluginAuthority,
    pub plugin: Plugin<'a>,
}

impl<'a> Serialize for HashablePluginSchema<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.index.serialize_to(buffer);
        offset += self.authority.serialize_to(&mut buffer[offset..]);
        offset += self.plugin.serialize_to(&mut buffer[offset..]);
        offset
    }
}

pub struct CompressionProof<'a> {
    pub owner: Pubkey,
    pub update_authority: UpdateAuthority,
    pub name: &'a [u8],
    pub uri: &'a [u8],
    pub seq: u64,
    pub plugins: &'a [HashablePluginSchema<'a>],
}

impl<'a> Serialize for CompressionProof<'a> {
    fn serialize_to(&self, buffer: &mut [u8]) -> usize {
        let mut offset = self.owner.serialize_to(buffer);
        offset += self.update_authority.serialize_to(&mut buffer[offset..]);
        offset += self.name.serialize_to(&mut buffer[offset..]);
        offset += self.uri.serialize_to(&mut buffer[offset..]);
        offset += self.seq.serialize_to(&mut buffer[offset..]);
        offset += self.plugins.serialize_to(&mut buffer[offset..]);
        offset
    }
}
