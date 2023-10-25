use crate::*;
use near_sdk::CryptoHash;
use std::mem::size_of;

// Convert the royalty percentage and amount to pay into a payout (U128).
pub(crate) fn royalty_to_payout(royalty_percentage: u32, amount_to_pay: Balance) -> U128 {
    U128(royalty_percentage as u128 * amount_to_pay / 10_000u128)
}

// Calculate how many bytes the account ID is taking up.
pub(crate) fn bytes_for_approved_account_id(account_id: &AccountId) -> u64 {
    // The extra 4 bytes are coming from Borsh serialization to store the length of the string.
    account_id.as_str().len() as u64 + 4 + size_of::<u64>() as u64
}

// Refund the storage taken up by passed-in approved account IDs and send the funds to the passed-in account ID.
pub(crate) fn refund_approved_account_ids_iter<'a, I>(
    account_id: AccountId,
    approved_account_ids: I, // The approved account IDs must be passed in as an iterator.
) -> Promise
where
    I: Iterator<Item = &'a AccountId>,
{
    // Get the storage total by going through and summing all the bytes for each approved account IDs.
    let storage_released: u64 = approved_account_ids.map(bytes_for_approved_account_id).sum();
    // Transfer the account the storage that is released.
    Promise::new(account_id).transfer(Balance::from(storage_released) * env::storage_byte_cost())
}

// Refund a map of approved account IDs and send the funds to the passed-in account ID.
pub(crate) fn refund_approved_account_ids(
    account_id: AccountId,
    approved_account_ids: &HashMap<AccountId, u64>,
) -> Promise {
    // Call the refund_approved_account_ids_iter with the approved account IDs as keys.
    refund_approved_account_ids_iter(account_id, approved_account_ids.keys())
}

// Used to generate a unique prefix in our storage collections (this is to avoid data collisions).
pub(crate) fn hash_account_id(account_id: &String) -> CryptoHash {
    // Get the default hash.
    let mut hash = CryptoHash::default();
    // We hash the account ID and return it.
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

// Used to make sure the user attached exactly 1 yoctoNEAR.
pub(crate) fn assert_one_yocto() {
    assert_eq!(
        env::attached_deposit(),
        1,
        "Requires attached deposit of exactly 1 yoctoNEAR",
    );
}

// Assert that the user has attached at least 1 yoctoNEAR (for security reasons and to pay for storage).
pub(crate) fn assert_at_least_one_yocto() {
    assert!(
        env::attached_deposit() >= 1,
        "Requires attached deposit of at least 1 yoctoNEAR",
    );
}

// Refund the initial deposit based on the amount of storage that was used up.
pub(crate) fn refund_deposit(storage_used: u64) {
    // Get how much it would cost to store the information.
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    // Get the attached deposit.
    let attached_deposit = env::attached_deposit();

    // Make sure that the attached deposit is greater than or equal to the required cost.
    assert!(
        required_cost <= attached_deposit,
        "Must attach {} yoctoNEAR to cover storage",
        required_cost,
    );

    // Get the refund amount from the attached deposit - required cost.
    let refund = attached_deposit - required_cost;

    // If the refund is greater than 1 yoctoNEAR, we refund the predecessor that amount.
    if refund > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund);
    }
}

impl Contract {
    // Add a ticket to the set of tickets an owner has.
    pub(crate) fn assert_contract_owner(&mut self) {
        assert!(
            self.owner_id == env::predecessor_account_id(),
            "only contract owner"
        )
    }

    // Add a set of tickets to the set of tickets an owner has.
    pub(crate) fn internal_add_tickets_to_owner(
        &mut self,
        account_id: &AccountId,
        ticket_id: &Vec<TicketId>,
    ) {
        // Get the set of tickets for the given account.
        let mut ticket_set = self.tickets_per_owner.get(account_id).unwrap_or_else(|| {
            // If the account doesn't have any tickets, we create a new unordered set.
            UnorderedSet::new(
                StorageKey::TicketsPerOwnerInner {
                    // Get a new unique prefix for the collection.
                    account_id_hash: hash_account_id(&account_id.to_string()),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        // Insert the tickets into the set.
        for ticket in ticket_id {
            ticket_set.insert(ticket);
        }

        // Insert that set for the given account ID.
        self.tickets_per_owner.insert(account_id, &ticket_set);
    }

    // Remove a ticket from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_remove_tickets_from_owner(
        &mut self,
        account_id: &AccountId,
        ticket_id: &Vec<TicketId>,
    ) {
        // Get the set of tickets that the owner has.
        let mut ticket_set = self
            .tickets_per_owner
            .get(account_id)
            // If there is no set of tickets for the owner, we panic with the following message:
            .expect("Ticket should be owned by the sender");

        // Remove the token_id from the set of tokens.
        for ticket in ticket_id {
            ticket_set.remove(ticket);
        }

        // If the ticket set is now empty, we remove the owner from the tickets_per_owner collection.
        if ticket_set.is_empty() {
            self.tickets_per_owner.remove(account_id);
        } else {
            // If the ticket set is not empty, we simply insert it back for the account ID.
            self.tickets_per_owner.insert(account_id, &ticket_set);
        }
    }

    // Transfer the ticket to the receiver_id.
    pub(crate) fn internal_transfer(
        &mut self,
        sender_id: &AccountId,
        receiver_id: &AccountId,
        ticket_id: TicketId,
        // We introduce an approval ID so that people with that approval ID can transfer the token.
        approval_id: Option<u64>,
        memo: Option<String>,
    ) -> Ticket {
        // Get the ticket object by passing in the ticket_id.
        let ticket = self.ticket_by_id.get(&ticket_id).expect("No token");

        // If the sender doesn't equal the owner, we check if the sender is in the approval list.
        if sender_id != &ticket.owner_id {
            // If the ticket's approved account IDs don't contain the sender, we panic.
            if !ticket.approved_account_ids.contains_key(sender_id) {
                env::panic_str("Unauthorized");
            }

            // If they included an approval_id, check if the sender's actual approval_id is the same as the one included.
            if let Some(enforced_approval_id) = approval_id {
                // Get the actual approval ID.
                let actual_approval_id = ticket
                    .approved_account_ids
                    .get(sender_id)
                    // If the sender isn't in the map, we panic.
                    .expect("Sender is not an approved account");

                // Make sure that the actual approval ID is the same as the one provided.
                assert_eq!(
                    actual_approval_id, &enforced_approval_id,
                    "The actual approval_id {} is different from the given approval_id {}",
                    actual_approval_id, enforced_approval_id,
                );
            }
        }

        // Make sure that the sender isn't sending the ticket to themselves.
        assert_ne!(
            &ticket.owner_id, receiver_id,
            "The ticket owner and the receiver should be different"
        );

        // Remove the ticket from its current owner's set.
        self.internal_remove_tickets_from_owner(&ticket.owner_id, &vec![ticket_id]);
        // Add the ticket to the receiver_id's set.
        self.internal_add_tickets_to_owner(receiver_id, &vec![ticket_id]);

        // Create a new ticket struct.
        let new_ticket = Ticket {
            raffle_id: ticket.raffle_id,
            owner_id: receiver_id.clone(),
            // Reset the approval account IDs.
            approved_account_ids: Default::default(),
            next_approval_id: ticket.next_approval_id,
        };
        // Insert that new ticket into the tickets_by_id, replacing the old entry.
        self.ticket_by_id.insert(&ticket_id, &new_ticket);

        // If there was some memo attached, we log it.
        if let Some(memo) = memo.as_ref() {
            env::log_str(&format!("Memo: {}", memo).to_string());
        }

        // Default the authorized ID to be None for the logs.
        let mut authorized_id = None;
        // If the approval ID was provided, set the authorized ID equal to the sender.
        if approval_id.is_some() {
            authorized_id = Some(sender_id.to_string());
        }

        // Construct the transfer log as per the events standard.
        let nft_transfer_log: EventLog = EventLog {
            // Standard name ("nep171").
            standard: NFT_STANDARD_NAME.to_string(),
            // Version of the standard ("nft-1.0.0").
            version: NFT_METADATA_SPEC.to_string(),
            // The data related with the event stored in a vector.
            event: EventLogVariant::NftTransfer(vec![NftTransferLog {
                // The optional authorized account ID to transfer the ticket on behalf of the old owner.
                authorized_id,
                // The old owner's account ID.
                old_owner_id: ticket.owner_id.to_string(),
                // The account ID of the new owner of the ticket.
                new_owner_id: receiver_id.to_string(),
                // A vector containing the ticket IDs as strings.
                token_ids: vec![ticket_id.to_string()],
                // An optional memo to include.
                memo,
            }]),
        };

        // Log the serialized JSON.
        env::log_str(&nft_transfer_log.to_string());

        // Return the previous ticket object that was transferred.
        ticket
    }
}
