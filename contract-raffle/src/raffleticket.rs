use crate::*;
pub type TicketNumber = i32;
pub type TicketId = u64;

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Ticket {
    id: TicketId,
    numbers: Vec<TicketNumber>,
    owner_id: Option<AccountId>,
    is_winning_ticket: bool,
}
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct RaffleTicket {
    winning_tickets: UnorderedMap<TicketId, Ticket>,
    available_tickets: UnorderedMap<TicketId, Ticket>,
    sold_tickets: UnorderedMap<AccountId, Ticket>,
    prizes_per_ticket: Balance,
    number_of_predefined: i16,
}

impl RaffleTicket {
    pub fn new(prizes_per_ticket: Balance, number_of_predefined: i16) -> Self {
        let mut raffle = RaffleTicket {
            available_tickets: UnorderedMap::new(StorageKey::Available),
            winning_tickets: UnorderedMap::new(StorageKey::Winning),
            sold_tickets: UnorderedMap::new(StorageKey::Sold),
            prizes_per_ticket,
            number_of_predefined,
        };
        raffle.reset();
        raffle
    }
    pub fn reset(&mut self) {
        self.available_tickets.clear();
        self.winning_tickets.clear();
        self.sold_tickets.clear();
        self.add_tickets(self.number_of_predefined, false);
        self.add_tickets_as_winning(1);
    }
    pub fn total_available(&self) -> u64 {
        self.available_tickets.len()
    }
    fn add_tickets_as_winning(&mut self, number_of_predefined: i16) {
        self.add_tickets(number_of_predefined, true);
    }

    fn add_tickets(&mut self, number_of_predefined: i16, is_winning_ticket: bool) {
        for _ in 0..number_of_predefined {
            let ticket = self.new_ticket(false, None);
            //            let idstr = ticket.id.to_string();
            if is_winning_ticket {
                self.winning_tickets.insert(&ticket.id, &ticket);
            } else {
                self.available_tickets.insert(&ticket.id, &ticket);
            }
        }
    }
    fn new_ticket(&mut self, is_winning_ticket: bool, owner_id: Option<AccountId>) -> Ticket {
        Ticket {
            id: self.available_tickets.len(),
            owner_id: owner_id,
            numbers: self.generate_ticket_numbers(),
            is_winning_ticket: is_winning_ticket,
        }
    }
    fn generate_ticket_numbers(&self) -> Vec<TicketNumber> {
        let numbers: Vec<_> = (0..5).map(|_| rand_range(100, 1000)).collect();
        return numbers;
    }

    pub fn buy_ticket(
        &mut self,
        buyer_id: AccountId,
        prize_tokens: Balance,
    ) -> Result<Balance, &str> {
        if prize_tokens < self.prizes_per_ticket.into() {
            return Err("Invalid prize amount");
        }
        if self.total_available() < 1 {
            return Err("No prize tickets available");
        }
        let mut refund = prize_tokens % self.prizes_per_ticket;
        let buy_count = prize_tokens / self.prizes_per_ticket;
        for t in 0..buy_count {
            if self.total_available() < 1 {
                let left = (buy_count - t) * self.prizes_per_ticket;
                refund = refund + left;
                break;
            } else {
                let key = self.available_tickets.keys().last().unwrap();
                let ticket = self.available_tickets.get(&key).expect("Ticket not found");
                self.sold_tickets.insert(&buyer_id, &ticket);
                self.available_tickets.remove(&key);
            }
        }

        Ok(refund)
    }
}
