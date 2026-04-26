#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Auction {
    pub seller: Address,
    pub item_name: String,
    pub description: String,
    pub starting_price: i128,
    pub highest_bid: i128,
    pub highest_bidder: Option<Address>,
    pub is_active: bool,
}

#[contracttype]
pub enum DataKey {
    Auction,
    Bid(Address),
}

#[contract]
pub struct StellarLiveAuction;

#[contractimpl]
impl StellarLiveAuction {
    pub fn create_auction(
        env: Env,
        seller: Address,
        item_name: String,
        description: String,
        starting_price: i128,
    ) {
        seller.require_auth();

        if starting_price <= 0 {
            panic!("Starting price must be greater than 0");
        }

        let auction = Auction {
            seller: seller.clone(),
            item_name,
            description,
            starting_price,
            highest_bid: starting_price,
            highest_bidder: None,
            is_active: true,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Auction, &auction);

        env.events().publish(
            (symbol_short!("CREATE"), seller),
            starting_price,
        );
    }

    pub fn place_bid(
        env: Env,
        bidder: Address,
        amount: i128,
    ) -> i128 {
        bidder.require_auth();

        if amount <= 0 {
            panic!("Bid amount must be greater than 0");
        }

        let mut auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction)
            .expect("Auction not found");

        if !auction.is_active {
            panic!("Auction is not active");
        }

        if amount <= auction.highest_bid {
            panic!("Bid must be higher than current highest bid");
        }

        auction.highest_bid = amount;
        auction.highest_bidder = Some(bidder.clone());

        env.storage()
            .persistent()
            .set(&DataKey::Auction, &auction);

        env.storage()
            .persistent()
            .set(&DataKey::Bid(bidder.clone()), &amount);

        env.events().publish(
            (symbol_short!("BID"), bidder),
            amount,
        );

        amount
    }

    pub fn close_auction(env: Env, seller: Address) {
        seller.require_auth();

        let mut auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction)
            .expect("Auction not found");

        if auction.seller != seller {
            panic!("Only seller can close auction");
        }

        auction.is_active = false;

        env.storage()
            .persistent()
            .set(&DataKey::Auction, &auction);

        env.events().publish(
            (symbol_short!("CLOSE"), seller),
            auction.highest_bid,
        );
    }

    pub fn get_auction(env: Env) -> Auction {
        env.storage()
            .persistent()
            .get(&DataKey::Auction)
            .expect("Auction not found")
    }

    pub fn get_highest_bid(env: Env) -> i128 {
        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction)
            .expect("Auction not found");

        auction.highest_bid
    }

    pub fn get_highest_bidder(env: Env) -> Option<Address> {
        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction)
            .expect("Auction not found");

        auction.highest_bidder
    }

    pub fn get_bid(env: Env, bidder: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Bid(bidder))
            .unwrap_or(0)
    }

    pub fn is_active(env: Env) -> bool {
        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction)
            .expect("Auction not found");

        auction.is_active
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_create_auction() {
        let env = Env::default();
        let contract_id = env.register(StellarLiveAuction, ());
        let client = StellarLiveAuctionClient::new(&env, &contract_id);

        let seller = Address::generate(&env);

        env.mock_all_auths();

        client.create_auction(
            &seller,
            &String::from_str(&env, "Stellar Builder Badge"),
            &String::from_str(&env, "A test auction item on Stellar"),
            &100,
        );

        let auction = client.get_auction();

        assert_eq!(auction.seller, seller);
        assert_eq!(auction.starting_price, 100);
        assert_eq!(auction.highest_bid, 100);
        assert_eq!(auction.is_active, true);
    }

    #[test]
    fn test_place_bid() {
        let env = Env::default();
        let contract_id = env.register(StellarLiveAuction, ());
        let client = StellarLiveAuctionClient::new(&env, &contract_id);

        let seller = Address::generate(&env);
        let bidder = Address::generate(&env);

        env.mock_all_auths();

        client.create_auction(
            &seller,
            &String::from_str(&env, "Stellar NFT"),
            &String::from_str(&env, "Auction item"),
            &100,
        );

        let result = client.place_bid(&bidder, &150);

        assert_eq!(result, 150);
        assert_eq!(client.get_highest_bid(), 150);
    }

    #[test]
    fn test_get_bid() {
        let env = Env::default();
        let contract_id = env.register(StellarLiveAuction, ());
        let client = StellarLiveAuctionClient::new(&env, &contract_id);

        let seller = Address::generate(&env);
        let bidder = Address::generate(&env);

        env.mock_all_auths();

        client.create_auction(
            &seller,
            &String::from_str(&env, "Stellar Auction Item"),
            &String::from_str(&env, "Live bidding demo"),
            &100,
        );

        client.place_bid(&bidder, &200);

        let bid = client.get_bid(&bidder);

        assert_eq!(bid, 200);
    }

    #[test]
    fn test_close_auction() {
        let env = Env::default();
        let contract_id = env.register(StellarLiveAuction, ());
        let client = StellarLiveAuctionClient::new(&env, &contract_id);

        let seller = Address::generate(&env);

        env.mock_all_auths();

        client.create_auction(
            &seller,
            &String::from_str(&env, "Auction Item"),
            &String::from_str(&env, "Auction description"),
            &100,
        );

        client.close_auction(&seller);

        assert_eq!(client.is_active(), false);
    }
}