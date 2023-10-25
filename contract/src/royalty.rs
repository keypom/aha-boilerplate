use near_sdk::json_types::U64;
use crate::*;

pub trait NonFungibleTokenCore {
    //calculates the payout for a token given the passed in balance. This is a view method
    fn nft_payout(&self, token_id: U64, balance: U128, max_len_payout: u32) -> Payout;

    //transfers the token to the receiver ID and returns the payout object that should be payed given the passed in balance.
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        token_id: U64,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout;
}

#[near_bindgen]
impl NonFungibleTokenCore for Contract {
    // Calculates the payout for a ticket given the passed-in balance. This is a view method.
    fn nft_payout(&self, ticket_id: U64, balance: U128, max_len_payout: u32) -> Payout {
        // Get the ticket object
        let ticket = self.ticket_by_id.get(&ticket_id.0).expect("No ticket");

        // Get the owner of the ticket
        let owner_id = ticket.owner_id;
        // Keep track of the total perpetual royalties
        let mut total_perpetual = 0;
        // Get the u128 version of the passed-in balance (which was U128 before)
        let balance_u128 = u128::from(balance);
        // Keep track of the payout object to send back
        let mut payout_object = Payout {
            payout: HashMap::new(),
        };
        // Get the royalty object from the ticket
        let raffle = self
            .raffle_by_id
            .get(&ticket.raffle_id)
            .expect("No raffle currently running");
        let royalty_option = raffle.royalty;
        if royalty_option.is_none() {
            let mut payout = HashMap::new();
            payout.insert(owner_id, balance);
            return Payout { payout: payout };
        }
        let royalty = royalty_option.unwrap();

        // Make sure we're not paying out to too many people (GAS limits this)
        assert!(
            royalty.len() as u32 <= max_len_payout,
            "Market cannot payout to that many receivers"
        );

        // Go through each key and value in the royalty object
        for (k, v) in royalty.iter() {
            // Get the key
            let key = k.clone();
            // Only insert into the payout if the key isn't the ticket owner (we add their payout at the end)
            if key != owner_id {
                payout_object
                    .payout
                    .insert(key, royalty_to_payout(*v, balance_u128));
                total_perpetual += *v;
            }
        }

        // Payout to the previous owner who gets 100% - total perpetual royalties
        payout_object.payout.insert(
            owner_id,
            royalty_to_payout(10000 - total_perpetual, balance_u128),
        );

        // Return the payout object
        payout_object
    }


    // Transfers the ticket to the receiver ID and returns the payout object that should be paid given the passed-in balance.
    #[payable]
    fn nft_transfer_payout(
        &mut self,
        receiver_id: AccountId,
        ticket_id: U64,
        approval_id: u64,
        memo: Option<String>,
        balance: U128,
        max_len_payout: u32,
    ) -> Payout {
        // Assert that the user attached 1 yocto NEAR for security reasons
        assert_one_yocto();
        // Get the sender ID
        let sender_id = env::predecessor_account_id();
        // Transfer the ticket to the passed-in receiver and get the previous ticket object back
        let previous_ticket =
            self.internal_transfer(&sender_id, &receiver_id, ticket_id.0, Some(approval_id), memo);

        // Refund the previous ticket owner for the storage used up by the previous approved account IDs
        refund_approved_account_ids(
            previous_ticket.owner_id.clone(),
            &previous_ticket.approved_account_ids,
        );

        // Get the owner of the ticket
        let owner_id = previous_ticket.owner_id;
        // Keep track of the total perpetual royalties
        let mut total_perpetual = 0;
        // Get the u128 version of the passed-in balance (which was U128 before)
        let balance_u128 = u128::from(balance);
        // Keep track of the payout object to send back
        let mut payout_object = Payout {
            payout: HashMap::new(),
        };

        // Get the royalty object from the ticket
        let cur_raffle = self
            .raffle_by_id
            .get(&previous_ticket.raffle_id)
            .expect("Not a series");
        let royalty_option = cur_raffle.royalty;
        if royalty_option.is_none() {
            let mut payout = HashMap::new();
            payout.insert(owner_id, balance);
            return Payout { payout: payout };
        }
        let royalty = royalty_option.unwrap();

        // Make sure we're not paying out to too many people (GAS limits this)
        assert!(
            royalty.len() as u32 <= max_len_payout,
            "Market cannot payout to that many receivers"
        );

        // Go through each key and value in the royalty object
        for (k, v) in royalty.iter() {
            // Get the key
            let key = k.clone();
            // Only insert into the payout if the key isn't the ticket owner (we add their payout at the end)
            if key != owner_id {
                payout_object
                    .payout
                    .insert(key, royalty_to_payout(*v, balance_u128));
                total_perpetual += *v;
            }
        }

        // Payout to the previous owner who gets 100% - total perpetual royalties
        payout_object.payout.insert(
            owner_id,
            royalty_to_payout(10000 - total_perpetual, balance_u128),
        );

        // Return the payout object
        payout_object
    }
}
