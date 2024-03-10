#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod polkability {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;

    // todo: add other events: on event result execution, on new bid.
    /// Executes then an event added.
    #[ink(event)]
    pub struct EventAdded {
        owner: Option<AccountId>,
        min_bid_amount: u128,
        topic: String
    }

    /// Probabilistic event to store.
    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct ProbabilisticEvent {
        finalized: bool,
        topic: String,
        identifier: String,
        min_bid_amount: u128
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    /// Probabilistic event bid to store.
    pub struct ProbabilisticEventBid {
        identifier: String,
        estimated_date: String,
        bid_amount: u128,
        author: AccountId
    }

    #[ink(storage)]
    pub struct Polkability {
        /// Store probabilistic events.
        events: Vec<ProbabilisticEvent>,
        event_bids: Vec<ProbabilisticEventBid>
    }

    impl Polkability {
        /// Initialise initial (default) contract storage.
        #[ink(constructor)]
        pub fn new() -> Self {
            Polkability {
                events: Vec::new(),
                event_bids: Vec::new()
            }
        }

        /// Add a new probabilistic event.
        #[ink(message, payable)]
        pub fn add_event(&mut self, e_identifier: String, topic: String) {
            // todo: prevent event with exists topic from pushing, implement return of all funds to the topic owner if nobody wins.
            let caller = Self::env().caller();
            let transferred_value = self.env().transferred_value();

            self.events.push(ProbabilisticEvent {
                topic: topic.clone(),
                min_bid_amount: self.env().transferred_value(),
                finalized: false,
                identifier: e_identifier
            });

            Self::env().emit_event(EventAdded {
                owner: Some(caller),
                min_bid_amount: transferred_value,
                topic: topic.clone()
            });
        }

        /// Place a new bid to probabilistic event.
        #[ink(message, payable)]
        pub fn add_bid_to_event(&mut self, e_identifier: String, estimated_date: String, from: AccountId) {
            let transferred_value = self.env().transferred_value();
            let related_event = self.events.clone().into_iter().find(|e| e.identifier == e_identifier).unwrap();

            if transferred_value > related_event.min_bid_amount {
                self.event_bids.push(ProbabilisticEventBid {
                    bid_amount: self.env().transferred_value(),
                    estimated_date: estimated_date,
                    author: from,
                    identifier: e_identifier
                });
            }
        }

        /// Handle event execution from oracle.
        #[ink(message)]
        pub fn dispatch_event(&mut self, estimated_date: String, topic: String) {
            // todo: secure oracle from which message is received.
            let related_event = self.events.clone().into_iter().find(|e| e.topic == topic && !e.finalized).unwrap();
        
            for e in self.events.iter_mut() {
                if e.identifier == related_event.identifier {
                    e.finalized = true;
                }
            }

            let event_winner_bid = self.event_bids.clone().into_iter().find(|e_bid| e_bid.identifier == related_event.identifier && e_bid.estimated_date == estimated_date).unwrap();
            let transfer_to_winner_amount = self.event_bids.clone().iter()
                .filter(|e_bid| e_bid.identifier == related_event.identifier)
                .map(|e_bid| e_bid. bid_amount)
                .sum();

            let _ = self.env().transfer(event_winner_bid.author, transfer_to_winner_amount);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            Ok(())
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;

        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            Ok(())
        }
    }
}
