// Import necessary dependencies and types
use crate::*;
use near_sdk::{ext_contract, Gas, PromiseResult, json_types::U64};
use std::collections::HashMap;

// Define gas constants for resolving transfer and NFT on transfer calls
const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas(10_000_000_000_000);
const GAS_FOR_NFT_ON_TRANSFER: Gas = Gas(25_000_000_000_000);

// Define the trait for NonFungibleTokenCore
pub trait NonFungibleTokenCore {
    // Transfers an NFT to a receiver ID
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: U64,
        approval_id: Option<u64>,
        memo: Option<String>,
    );

    // Transfers an NFT to a receiver and calls a function on the receiver ID's contract
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: U64,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool>;

    // Get information about the NFT token
    fn nft_token(&self, token_id: U64) -> Option<JsonTicket>;
}

// External contract trait for NonFungibleTokenReceiver
#[ext_contract(ext_non_fungible_token_receiver)]
trait NonFungibleTokenReceiver {
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: U64,
        msg: String,
    ) -> Promise;
}

// External contract trait for NonFungibleTokenResolver
#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {
    fn nft_resolve_transfer(
        &mut self,
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: U64,
        approved_account_ids: HashMap<AccountId, u64>,
        memo: Option<String>,
    ) -> bool;
}

// Implement NonFungibleTokenCore for the Contract
#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    // Implementation of the nft_transfer method.
    #[payable]
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: U64,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        // Ensure the user attached exactly 1 yoctoNEAR for security and wallet redirection.
        assert_one_yocto();

        // Get the sender to transfer the token from the sender to the receiver
        let sender_id = env::predecessor_account_id();

        // Call the internal transfer method and get back the previous token to refund approved account IDs
        let previous_ticket =
            self.internal_transfer(&sender_id, &receiver_id, token_id.0, approval_id, memo);

        // Refund the owner for releasing the storage used up by approved account IDs
        refund_approved_account_ids(
            previous_ticket.owner_id.clone(),
            &previous_ticket.approved_account_ids,
        );
    }

    // Implementation of the transfer call method.
    #[payable]
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: U64,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        // Ensure the user attached exactly 1 yocto for security reasons.
        assert_one_yocto();

        // Get the sender ID
        let sender_id = env::predecessor_account_id();

        // Transfer the token and get the previous token object
        let previous_ticket = self.internal_transfer(
            &sender_id,
            &receiver_id,
            token_id.0,
            approval_id,
            memo.clone(),
        );

        // Default the authorized_id to none
        let mut authorized_id = None;

        // If the sender isn't the owner of the token, set the authorized ID equal to the sender.
        if sender_id != previous_ticket.owner_id {
            authorized_id = Some(sender_id.to_string());
        }

        // Initiating receiver's call and the callback
        ext_non_fungible_token_receiver::ext(receiver_id.clone())
            .with_static_gas(GAS_FOR_NFT_ON_TRANSFER)
            .nft_on_transfer(
                sender_id,
                previous_ticket.owner_id.clone(),
                token_id.clone(),
                msg,
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_RESOLVE_TRANSFER)
                    .nft_resolve_transfer(
                        authorized_id,
                        previous_ticket.owner_id,
                        receiver_id,
                        token_id,
                        previous_ticket.approved_account_ids,
                        memo,
                    ),
            )
            .into()
    }

    // Get information for a specific token ID
    fn nft_token(&self, token_id: U64) -> Option<JsonTicket> {
        // If there is a token with the provided ID in the collection
        if let Some(ticket) = self.ticket_by_id.get(&token_id.0) {
            let cur_series = self
                .raffle_by_id
                .get(&ticket.raffle_id)
                .expect("No raffle found");
            let mut metadata = cur_series.metadata;

            // Return the JsonToken wrapped in Some since we return an option
            Some(JsonTicket {
                raffle_id: U64(ticket.raffle_id),
                ticket_id: token_id,
                owner_id: ticket.owner_id,
                metadata,
                approved_account_ids: ticket.approved_account_ids,
                royalty: cur_series.royalty,
            })
        } else {
            // If the token with the provided ID doesn't exist, return None
            None
        }
    }
}

// Implement NonFungibleTokenResolver for the Contract
#[near_bindgen]
impl NonFungibleTokenResolver for Contract {
    // Resolve the cross-contract call when calling nft_on_transfer in the nft_transfer_call method
    // Return true if the token was successfully transferred to the receiver_id
    #[private]
    fn nft_resolve_transfer(
        &mut self,
        authorized_id: Option<String>,
        owner_id: AccountId,
        receiver_id: AccountId,
        token_id: U64,
        approved_account_ids: HashMap<AccountId, u64>,
        memo: Option<String>,
    ) -> bool {
        // Check whether the receiver wants to return the token back to the sender based on nft_on_transfer result.
        if let PromiseResult::Successful(value) = env::promise_result(0) {
            if let Ok(return_token) = near_sdk::serde_json::from_slice::<bool>(&value) {
                if !return_token {
                    // We don't need to return the token, so everything went fine, and we return true.
                    refund_approved_account_ids(owner_id, &approved_account_ids);
                    return true;
                }
            }
        }

        // Get the token object if it exists
        let mut token = if let Some(token) = self.ticket_by_id.get(&token_id.0) {
            if token.owner_id != receiver_id {
                refund_approved_account_ids(owner_id, &approved_account_ids);
                // The token is no longer owned by the receiver. Can't return it.
                return true;
            }
            token
        } else {
            refund_approved_account_ids(owner_id, &approved_account_ids);
            return true;
        };

        // Remove the token from the receiver
        self.internal_remove_tickets_from_owner(&receiver_id.clone(), &vec![token_id.0]);
        // Add the token to the original owner
        self.internal_add_tickets_to_owner(&owner_id, &vec![token_id.0]);

        // Change the token struct's owner to be the original owner
        token.owner_id = owner_id.clone();

        // Refund the receiver for any approved account IDs that they may have set on the token
        refund_approved_account_ids(receiver_id.clone(), &token.approved_account_ids);
        // Reset the approved account IDs to what they were before the transfer
        token.approved_account_ids = approved_account_ids;

        // Insert the token back into the tokens_by_id collection
        self.ticket_by_id.insert(&token_id.0, &token);

        // Log that the NFT was reverted back to the original owner
        let nft_transfer_log: EventLog = EventLog {
            standard: NFT_STANDARD_NAME.to_string(),
            version: NFT_METADATA_SPEC.to_string(),
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                authorized_id,
                old_owner_id: receiver_id.to_string(),
                new_owner_id: owner_id.to_string(),
                token_ids: vec![token_id.0.to_string()],
                memo,
            }]),
        };

        env::log_str(&nft_transfer_log.to_string());

        // Return false
        false
    }
}
