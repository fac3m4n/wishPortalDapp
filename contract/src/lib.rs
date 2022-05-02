mod model;
mod utils;

use crate::{
    model::{Donation, Wish},
    utils::{assert_self, assert_single_promise_success, AccountId, ONE_NEAR},
};

// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    Promise,
};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen, PromiseIndex};
near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]

pub struct Contract {
    owner: AccountId,
    wishes: Vec<Wish>,
    donations: Vec<Donation>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(owner: AccountId) -> Self {
        let wishes: Vec<Wish> = Vec::new();
        let donations: Vec<Donation> = Vec::new();

        Contract {
            owner,
            wishes,
            donations,
        }
    }

    pub fn add_wish(&mut self, title: String, donate: u128, description: String) {
        let id = self.wishes.len() as i32;

        self.wishes.push(Wish::new(id, title, donate, description));
        env::log("Added a new wish".as_bytes());
    }

    pub fn list_wishes(&self) -> Vec<Wish> {
        let wishes = &self.wishes;

        return wishes.to_vec();
    }

    pub fn wish_count(&mut self) -> usize {
        return self.wishes.len();
    }

    pub fn add_vote(&mut self, id: usize) {
        let wish: &mut Wish = self.wishes.get_mut(id).unwrap();
        let voter = env::predecessor_account_id();

        wish.total_votes = wish.total_votes + 1;
        env::log("vote submitted succesfully".as_bytes());
        wish.votes.push(voter);
    }

    pub fn add_donation(&mut self, id: usize, amount: u128) {
        let transfer_amount: u128 = ONE_NEAR * amount;

        let wish: &mut Wish = self.wishes.get_mut(id).unwrap();

        wish.total_donations = wish.total_donations + transfer_amount;
        self.donations.push(Donation::new());

        Promise::new(env::predecessor_account_id()).transfer(transfer_amount);

        env::log("You have donated succesfully".as_bytes());
    }

    pub fn get_total_donations(&mut self, id: usize) -> u128 {
        let wish: &mut Wish = self.wishes.get_mut(id).unwrap();
        return wish.total_donations;
    }
}
