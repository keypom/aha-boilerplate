// Import necessary dependencies and types
use crate::*;
use near_sdk::{ext_contract, json_types::U64};

// Define the trait for NonFungibleTokenCore
pub trait NonFungibleTokenCore {
    // Approve an account ID to transfer a token on your behalf
    fn nft_approve(&mut self, token_id: U64, account_id: AccountId, msg: Option<String>);

    // Check if the passed-in account has access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: U64,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool;

    // Revoke a specific account from transferring the token on your behalf
    fn nft_revoke(&mut self, token_id: U64, account_id: AccountId);

    // Revoke all accounts from transferring the token on your behalf
    fn nft_revoke_all(&mut self, token_id: U64);
}

// External contract trait for NonFungibleTokenApprovalsReceiver
#[ext_contract(ext_non_fungible_approval_receiver)]
trait NonFungibleTokenApprovalsReceiver {
    // Cross-contract call to an external contract that is initiated during nft_approve
    fn nft_on_approve(
        &mut self,
        token_id: U64,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    );
}

// Implement NonFungibleTokenCore for the Contract
#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    // Allow a specific account ID to approve a token on your behalf
    #[payable]
    fn nft_approve(&mut self, token_id: U64, account_id: AccountId, msg: Option<String>) {
        // Ensure at least one yocto is attached for security reasons, causing a redirect to the NEAR wallet.
        assert_at_least_one_yocto();

        // Get the token object from the token ID
        let mut ticket = self.ticket_by_id.get(&token_id.0).expect("No token");

        // Ensure that the caller of the function is the owner of the token
        assert_eq!(
            &env::predecessor_account_id(),
            &ticket.owner_id,
            "Predecessor must be the token owner."
        );

        // Get the next approval ID if we need a new approval
        let approval_id: u64 = ticket.next_approval_id;

        // Check if the account has been approved already for this token
        let is_new_approval = ticket
            .approved_account_ids
            .insert(account_id.clone(), approval_id)
            .is_none();

        // Calculate the storage used for the new approval if it's a new approval
        let storage_used = if is_new_approval {
            bytes_for_approved_account_id(&account_id)
        } else {
            0
        };

        // Increment the token's next approval ID by 1
        ticket.next_approval_id += 1;

        // Insert the token back into the tokens_by_id collection
        self.ticket_by_id.insert(&token_id.0, &ticket);

        // Refund any excess storage attached by the user. If the user didn't attach enough, panic.
        refund_deposit(storage_used);

        // If a message was passed into the function, initiate a cross-contract call on the
        // account to which access is granted.
        if let Some(msg) = msg {
            ext_non_fungible_approval_receiver::ext(account_id)
                .nft_on_approve(token_id, ticket.owner_id, approval_id, msg)
                .as_return();
        }
    }

    // Check if the passed-in account has access to approve the token ID
    fn nft_is_approved(
        &self,
        token_id: U64,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        // Get the token object from the token_id
        let ticket = self.ticket_by_id.get(&token_id.0).expect("No token");

        // Get the approval number for the passed-in account ID
        let approval = ticket.approved_account_ids.get(&approved_account_id);

        if let Some(approval) = approval {
            if let Some(approval_id) = approval_id {
                // Return if the approval ID passed in matches the actual approval ID for the account
                approval_id == *approval
            } else {
                true
            }
        } else {
            false
        }
    }

    // Revoke a specific account from transferring the token on your behalf
    #[payable]
    fn nft_revoke(&mut self, token_id: U64, account_id: AccountId) {
        // Ensure the user attached exactly 1 yoctoNEAR for security reasons
        assert_one_yocto();

        // Get the token object using the passed-in token_id
        let mut ticket = self.ticket_by_id.get(&token_id.0).expect("No token");

        // Get the caller of the function and assert that they are the owner of the token
        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &ticket.owner_id);

        // If the account ID was in the token's approvals, remove it, and refund the funds released by removing the approved account ID to the caller of the function
        if ticket.approved_account_ids.remove(&account_id).is_some() {
            refund_approved_account_ids_iter(predecessor_account_id, [account_id].iter());

            // Insert the token back into the tokens_by_id collection with the account_id removed from the approval list
            self.ticket_by_id.insert(&token_id.0, &ticket);
        }
    }

    // Revoke all accounts from transferring the token on your behalf
    #[payable]
    fn nft_revoke_all(&mut self, token_id: U64) {
        // Ensure the caller attached exactly 1 yoctoNEAR for security
        assert_one_yocto();

        // Get the token object from the passed-in token ID
        let mut ticket = self.ticket_by_id.get(&token_id.0).expect("No token");

        // Get the caller and make sure they are the owner of the tokens
        let predecessor_account_id = env::predecessor_account_id();
        assert_eq!(&predecessor_account_id, &ticket.owner_id);

        // Only revoke if the approved account IDs for the token are not empty
        if !ticket.approved_account_ids.is_empty() {
            // Refund the approved account IDs to the caller of the function
            refund_approved_account_ids(predecessor_account_id, &ticket.approved_account_ids);

            // Clear the approved account IDs
            ticket.approved_account_ids.clear();

            // Insert the token back into the tokens_by_id collection with the approved account IDs cleared
            self.ticket_by_id.insert(&token_id.0, &ticket);
        }
    }
}
