use near_sdk::json_types::U64;

use crate::*;

/// Injected Keypom Args struct to be sent to external contracts
#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct KeypomArgs {
    pub account_id_field: Option<String>,
    pub funder_id_field: Option<String>,
    pub drop_id_field: Option<String>,
    pub key_id_field: Option<String>
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn create_raffle(
        &mut self,
        raffle_id: u64,
        raffle_metadata: RaffleMetadata,
        royalty: Option<HashMap<AccountId, u32>>,
    ) {
        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let caller = env::predecessor_account_id();

        require!(
            self.raffle_by_id
                .get(&raffle_id)
                .is_none(),
            &format!(
                "raffle with ID {} already exists",
                &raffle_id
            )
        );

        require!(
            self.raffle_by_id
                .insert(
                    &raffle_id,
                    &Raffle {
                        metadata: raffle_metadata,
                        //we add an optional parameter for perpetual royalties
                        royalty,
                        tickets: UnorderedSet::new(StorageKey::RaffleTickets {
                            // We get a new unique prefix for the collection
                            raffle_id_hash: hash_account_id(&format!("{}{}", raffle_id, caller)),
                        }),
                        owner_id: caller
                    }
                )
                .is_none(),
            &format!(
                "raffle with ID {} already exists",
                &raffle_id
            )
        );

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }

    #[payable]
    pub fn mint_ticket(&mut self, raffle_id: U64, receiver_id: AccountId, ticket_amount: u64, drop_id: String, funder_id: AccountId, keypom_args: KeypomArgs) {
        // Ensure the injected keypom args are not malicious
        require!(keypom_args.funder_id_field.unwrap() == "funder_id".to_string(), "malicious call. Injected keypom args don't match");
        require!(keypom_args.drop_id_field.unwrap() == "drop_id".to_string(), "malicious call. Injected keypom args don't match");
        require!(keypom_args.account_id_field.unwrap() == "receiver_id".to_string(), "malicious call. Injected keypom args don't match");

        //measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let predecessor = env::predecessor_account_id();
        assert!(
            self.approved_minters.contains(&predecessor),
            "Not approved minter"
        );

        let mut raffle = self.raffle_by_id.get(&raffle_id.0).expect("Not a raffle");
        require!(raffle.drop_id.unwrap_or(drop_id) == drop_id, "drop_id mismatch");
        require!(raffle.funder_id.unwrap_or(funder_id) == funder_id, "funder_id mismatch");

        let cur_len = raffle.tickets.len() - 1 + ticket_amount;
        // Ensure we haven't overflowed on the number of copies minted
        if let Some(max) = raffle.metadata.max_tickets {
            require!(
                cur_len < max,
                "cannot mint anymore tickets for the given raffle. Limit reached"
            );
        }

        for _ in 0..ticket_amount {
            let ticket_id = raffle.tickets.len();

            raffle.tickets.insert(&ticket_id);
            self.ticket_id_per_owner.insert(&receiver_id, &ticket_id);

            // Construct the mint log as per the events standard.
            let nft_mint_log: EventLog = EventLog {
                // Standard name ("nep171").
                standard: NFT_STANDARD_NAME.to_string(),
                // Version of the standard ("nft-1.0.0").
                version: NFT_METADATA_SPEC.to_string(),
                // The data related with the event stored in a vector.
                event: EventLogVariant::NftMint(vec![NftMintLog {
                    // Owner of the token.
                    owner_id: receiver_id.to_string(),
                    // Vector of token IDs that were minted.
                    token_ids: vec![ticket_id.to_string()],
                    // An optional memo to include.
                    memo: None,
                }]),
            };

            // Log the serialized json.
            env::log_str(&nft_mint_log.to_string());
        }

        let token_id = format!("{}:{}", series_id, cur_len + 1);
        series.tokens.insert(&token_id);
        self.series_by_id.insert(&series_id, &series);

        //specify the token struct that contains the owner ID
        let token = Token {
            // Series ID that the token belongs to
            series_id,
            //set the owner ID equal to the receiver ID passed into the function
            owner_id: receiver_id,
            //we set the approved account IDs to the default value (an empty map)
            approved_account_ids: Default::default(),
            //the next approval ID is set to 0
            next_approval_id: 0,
        };

        //insert the token ID and token struct and make sure that the token doesn't exist
        require!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        //call the internal method for adding the token to the owner
        self.internal_add_token_to_owner(&token.owner_id, &token_id);

        // Construct the mint log as per the events standard.
        let nft_mint_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftMint(vec![NftMintLog {
                // Owner of the token.
                owner_id: token.owner_id.to_string(),
                // Vector of token IDs that were minted.
                token_ids: vec![token_id.to_string()],
                // An optional memo to include.
                memo: None,
            }]),
        };

        // Log the serialized json.
        env::log_str(&nft_mint_log.to_string());

        //calculate the required storage which was the used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        //refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }

    #[payable]
    /// Update the series ID for a given series. Caller must be series owner.
    pub fn update_mint_id(&mut self, old_mint_id: u64, new_mint_id: u64) {
        let caller = env::predecessor_account_id();
        // Ensure the caller is the owner of the current series

        let series_id = self
            .series_id_by_mint_id
            .remove(&old_mint_id)
            .expect("mint_id record not found");
        let mut series = self.series_by_id.get(&series_id).expect("Not a series");
        require!(
            series.owner_id == caller,
            "Only the owner can add a mint_id for this series_id"
        );

        // Add the series to the new ID and make sure the new ID doesn't exist yet
        require!(
            self.series_id_by_mint_id
                .insert(&new_mint_id, &series_id)
                .is_none(),
            &format!(
                "mint_id {} already exists and points to {}",
                &new_mint_id, &series_id
            )
        );

        series.mint_id = new_mint_id;
        self.series_by_id.insert(&series_id, &series);
    }
}
