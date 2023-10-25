use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, LookupSet, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, require, AccountId, Balance, BorshStorageKey, CryptoHash, PanicOnDefault,
    Promise, PromiseOrValue,
};
use std::collections::HashMap;

pub use crate::approval::*;
pub use crate::events::*;
use crate::internal::*;
pub use crate::metadata::*;
pub use crate::nft_core::*;
pub use crate::owner::*;
pub use crate::royalty::*;
pub use crate::raffle::*;

mod approval;
mod events;
mod internal;
mod metadata;
mod nft_core;
mod owner;
mod royalty;
mod raffle;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

// Represents the raffle type. All tokens will derive this data.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Raffle {
    // If specified, the drop funder coming from Keypom MUST be equal to this value
    funder_id: Option<AccountId>,
    // If specified, the drop ID coming from Keypom MUST be equal to this value
    drop_id: Option<String>,
    // Metadata including title, num copies, etc., that all tokens will derive from
    metadata: RaffleMetadata,
    // Royalty used for all tokens in the collection
    royalty: Option<HashMap<AccountId, u32>>,
    // Set of tickets in the collection
    tickets: UnorderedSet<TicketId>,
    // Owner of the raffle
    owner_id: AccountId
}

pub type CollectionId = u64;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // Contract owner
    pub owner_id: AccountId,
    // Approved minters
    pub approved_minters: LookupSet<AccountId>,
    // Approved users that can create raffles
    pub approved_creators: LookupSet<AccountId>,
    // Map the collection ID (stored in Token obj) to the collection data
    pub raffle_by_id: UnorderedMap<CollectionId, Raffle>,
    // Keeps track of the token struct for a given token ID
    pub ticket_by_id: UnorderedMap<TicketId, Ticket>,

    // Keeps track of all the token IDs for a given account
    pub tickets_per_owner: LookupMap<AccountId, UnorderedSet<TicketId>>,
    // Keeps track of the metadata for the contract
    pub metadata: LazyOption<NFTContractMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    ApprovedMinters,
    ApprovedCreators,
    RaffleById,
    RaffleByTicketId,
    TicketById,
    RaffleTickets { raffle_id_hash: CryptoHash },
    TicketsPerOwner,
    TicketsPerOwnerInner { account_id_hash: CryptoHash },
    Metadata,
}

#[near_bindgen]
impl Contract {
    // Initialization function (can only be called once).
    // This initializes the contract with default metadata so the
    // user doesn't have to manually type metadata.
    #[init]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: "nft-1.0.0".to_string(),
                name: "NFT Tutorial Contract".to_string(),
                symbol: "GOTEAM".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    // Initialization function (can only be called once).
    // This initializes the contract with metadata that was passed in and
    // the owner_id.
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        let mut approved_minters =
            LookupSet::new(StorageKey::ApprovedMinters.try_to_vec().unwrap());
        approved_minters.insert(&owner_id);

        let mut approved_creators =
            LookupSet::new(StorageKey::ApprovedCreators.try_to_vec().unwrap());
        approved_creators.insert(&owner_id);

        let this = Self {
            owner_id,
            approved_minters,
            approved_creators,
            raffle_by_id: UnorderedMap::new(StorageKey::RaffleById.try_to_vec().unwrap()),
            ticket_by_id: UnorderedMap::new(
                StorageKey::TicketById.try_to_vec().unwrap(),
            ),
            tickets_per_owner: LookupMap::new(
                StorageKey::TicketsPerOwner.try_to_vec().unwrap(),
            ),
            metadata: LazyOption::new(
                StorageKey::Metadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
        };

        this
    }
}
