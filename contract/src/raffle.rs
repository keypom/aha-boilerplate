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
        funder_id: Option<AccountId>,
        drop_id: Option<String>,
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
                        funder_id,
                        drop_id,
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
    pub fn mint_ticket(
        &mut self,
        raffle_id: U64,
        receiver_id: AccountId,
        ticket_amount: u64,
        drop_id: String,
        funder_id: AccountId,
        keypom_args: KeypomArgs,
    ) {
        // Ensure the injected keypom args are not malicious
        require!(
            keypom_args.funder_id_field.unwrap() == "funder_id".to_string(),
            "Malicious call. Injected keypom args don't match"
        );
        require!(
            keypom_args.drop_id_field.unwrap() == "drop_id".to_string(),
            "Malicious call. Injected keypom args don't match"
        );
        require!(
            keypom_args.account_id_field.unwrap() == "receiver_id".to_string(),
            "Malicious call. Injected keypom args don't match"
        );

        // Measure the initial storage being used on the contract
        let initial_storage_usage = env::storage_usage();

        let predecessor = env::predecessor_account_id();
        assert!(
            self.approved_minters.contains(&predecessor),
            "Not approved minter"
        );

        let mut raffle = self.raffle_by_id.get(&raffle_id.0).expect("Not a raffle");
        require!(
            raffle.drop_id.as_ref() == Some(drop_id).as_ref(),
            "drop_id mismatch"
        );
        require!(
            raffle.funder_id.as_ref() == Some(funder_id).as_ref(),
            "funder_id mismatch"
        );

        let cur_len = raffle.tickets.len() - 1 + ticket_amount;
        // Ensure we haven't overflowed on the number of copies minted
        if let Some(max) = raffle.metadata.max_tickets {
            require!(
                cur_len < max,
                "Cannot mint any more tickets for the given raffle. Limit reached"
            );
        }

        let mut tickets = vec![];
        for _ in 0..ticket_amount {
            let ticket_id = raffle.tickets.len();
            tickets.push(ticket_id);

            raffle.tickets.insert(&ticket_id);
            // Specify the ticket struct that contains the owner ID
            let ticket = Ticket {
                // Series ID that the ticket belongs to
                raffle_id: raffle_id.0,
                // Set the owner ID equal to the receiver ID passed into the function
                owner_id: receiver_id.clone(),
                // Set the approved account IDs to the default value (an empty map)
                approved_account_ids: Default::default(),
                // The next approval ID is set to 0
                next_approval_id: 0,
            };

            require!(
                self.ticket_by_id.insert(&ticket_id, &ticket).is_none(),
                "Ticket already exists"
            );

            // Construct the mint log as per the events standard.
            let nft_mint_log: EventLog = EventLog {
                // Standard name ("nep171")
                standard: NFT_STANDARD_NAME.to_string(),
                // Version of the standard ("nft-1.0.0")
                version: NFT_METADATA_SPEC.to_string(),
                // The data related with the event stored in a vector
                event: EventLogVariant::NftMint(vec![NftMintLog {
                    // Owner of the ticket
                    owner_id: receiver_id.to_string(),
                    // Vector of ticket IDs that were minted
                    token_ids: vec![ticket_id.to_string()],
                    // An optional memo to include
                    memo: None,
                }]),
            };

            // Log the serialized JSON
            env::log_str(&nft_mint_log.to_string());
        }
        self.internal_add_tickets_to_owner(&receiver_id, &tickets);
        self.raffle_by_id.insert(&raffle_id.0, &raffle);

        // Calculate the required storage which was used - initial
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;

        // Refund any excess storage if the user attached too much. Panic if they didn't attach enough to cover the required.
        refund_deposit(required_storage_in_bytes);
    }
}
