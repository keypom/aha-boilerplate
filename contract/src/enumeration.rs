use crate::nft_core::NonFungibleTokenCore;
use crate::*;

/// Struct to return in views to query for specific data related to an access key.
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonSeries {
    series_id: u64,
    mint_id: u64,
    // Metadata including title, num copies etc.. that all tokens will derive from
    metadata: TokenMetadata,
    // Royalty used for all tokens in the collection
    royalty: Option<HashMap<AccountId, u32>>,
    // Owner of the collection (they can update collection ID)
    owner_id: AccountId,
}

#[near_bindgen]
impl Contract {
    //Query for the total supply of NFTs on the contract
    pub fn nft_total_supply(&self) -> U128 {
        //return the length of the token metadata by ID
        U128(self.tokens_by_id.len() as u128)
    }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonToken> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each token using an iterator
        self.tokens_by_id
            .keys()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            U128(tokens_for_owner_set.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        //get the set of tokens for the passed in owner
        let tokens_for_owner_set = self.tokens_per_owner.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let tokens = if let Some(tokens_for_owner_set) = tokens_for_owner_set {
            tokens_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector.
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        tokens
            .iter()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    // Get the total supply of NFTs for a given owner
    pub fn get_supply_series(&self) -> u64 {
        self.series_by_id.len()
    }

    // Paginate through all the series on the contract and return the metadata for each one
    pub fn get_series(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonSeries> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each series using an iterator
        self.series_by_id
            .keys()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            //we'll map the series IDs which are strings into Json Series
            .map(|series_id| self.get_series_info(None, Some(series_id.clone())).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    // get info for a specific series
    pub fn get_series_info(
        &self,
        mint_id: Option<u64>,
        series_id: Option<u64>,
    ) -> Option<JsonSeries> {
        // If a series ID was passed in, use that. Otherwise, get the series ID from the mint ID.
        let actual_id = series_id.unwrap_or_else(|| {
            self.series_id_by_mint_id
                .get(&mint_id.expect("No mint ID or series ID passed in"))
                .expect("No series ID found for mint ID")
        });

        // If there was some series, return the series info
        if let Some(series) = self.series_by_id.get(&actual_id) {
            Some(JsonSeries {
                series_id: actual_id,
                mint_id: series.mint_id,
                metadata: series.metadata,
                royalty: series.royalty,
                owner_id: series.owner_id,
            })
        } else {
            //if there isn't a series, we'll return None
            None
        }
    }

    //get the total supply of NFTs for a given owner
    pub fn nft_supply_for_series(&self, mint_id: Option<u64>, series_id: Option<u64>) -> U128 {
        // If a series ID was passed in, use that. Otherwise, get the series ID from the mint ID.
        let actual_id = series_id.unwrap_or_else(|| {
            self.series_id_by_mint_id
                .get(&mint_id.expect("No mint ID or series ID passed in"))
                .expect("No series ID found for mint ID")
        });

        let series = self.series_by_id.get(&actual_id);

        //if there is some series, get the length of the tokens. Otherwise return -
        if let Some(series) = series {
            U128(series.tokens.len() as u128)
        } else {
            U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn nft_tokens_for_series(
        &self,
        mint_id: Option<u64>,
        series_id: Option<u64>,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        // If a series ID was passed in, use that. Otherwise, get the series ID from the mint ID.
        let actual_id = series_id.unwrap_or_else(|| {
            self.series_id_by_mint_id
                .get(&mint_id.expect("No mint ID or series ID passed in"))
                .expect("No series ID found for mint ID")
        });

        let series = self.series_by_id.get(&actual_id);
        
        let tokens = if let Some(series) = series {
            series.tokens
        } else {
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        tokens
            .iter()
            //skip to the index we specified in the start variable
            .skip(start as usize)
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize)
            //we'll map the token IDs which are strings into Json Tokens
            .map(|token_id| self.nft_token(token_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }
}
