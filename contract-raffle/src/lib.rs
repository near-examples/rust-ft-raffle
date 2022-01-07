mod raffleticket;

use crate::raffleticket::RaffleTicket;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::PromiseOrValue;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault};
use std::str::FromStr;

pub(crate) fn assert_initialized() {
    assert!(!env::state_exists(), "Already initialized");
}
pub(crate) fn rand_range(from: i32, to: i32) -> i32 {
    let seed=env::random_seed();
    let x: u32 = 123456789 ^ u32::from(seed[0]);
    let m = (to - from + 1) as u32;
    let t = x ^ x.wrapping_shl(11);
    let mut w: u32 = 88675123;
    w ^= w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
    return from + (w % m) as i32;
}

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Available,
    Winning,
    Sold,
}
#[derive(Debug, PartialEq)]
enum RaffleInstruction {
    BuyPrize,
    BuyTicket,
}

impl FromStr for RaffleInstruction {
    type Err = ();

    fn from_str(input: &str) -> Result<RaffleInstruction, Self::Err> {
        match input {
            "buy_ticket" => Ok(RaffleInstruction::BuyTicket),
            "buy_prize" => Ok(RaffleInstruction::BuyPrize),
            _ => Err(()),
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]

pub struct RaffleContract {
    ticket: RaffleTicket,
    fungible_token_account_id: AccountId,
}

#[near_bindgen]
impl RaffleContract {
    #[init]
    pub fn new(
        fungible_token_account_id: AccountId,
        tokens_per_ticket: Balance,
        number_of_predefined: i16,
    ) -> Self {
        assert_initialized();
        Self {
            ticket: RaffleTicket::new(tokens_per_ticket, number_of_predefined),
            fungible_token_account_id,
        }
    }

    pub fn total_tickets(&self) -> u64 {
        self.ticket.total_available()
    }
    pub fn reset(&mut self) {
        self.ticket.reset();
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for RaffleContract {
    /// If given `msg: "take-my-money", immediately returns U128::From(0)
    /// Otherwise, makes a cross-contract call to own `value_please` function, passing `msg`
    /// value_please will attempt to parse `msg` as an integer and return a U128 version of it
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Verifying that we were called by fungible token contract that we expect.
        assert!(
            env::predecessor_account_id() == self.fungible_token_account_id,
            "Only supports the one fungible token contract"
        );
        log!(
            "in {} tokens from @{} ft_on_transfer, msg = {}",
            amount.0,
            sender_id.as_ref(),
            msg
        );
        match msg.parse().unwrap() {
            RaffleInstruction::BuyTicket => {
                let result = self.ticket.buy_ticket(sender_id.into(), amount.into());
                match result {
                    Ok(s) => PromiseOrValue::Value(s.into()),
                    Err(e) => {
                        log!(e);
                        PromiseOrValue::Value(amount)
                    }
                }
            }
            _ => {
                log!("Invalid instruction for raffle call");
                PromiseOrValue::Value(amount)
            }
        }
    }
}
