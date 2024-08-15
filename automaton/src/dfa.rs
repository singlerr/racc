use std::collections::HashMap;
use std::ops::Deref;
use regex::Regex;

pub enum AutomataResponse{
    ACCEPT,
    REJECT
}

/// The first snapshot of DFA would be StateStorage based on HashMap
/// TODO("Is there better state-symbol encoding?")
pub struct DFA<T: StateStorage, F: AcceptanceChecker>{
    /// Current state id of this machine.
    /// Initial value must be 0 because in tradition, initial state id is 0.
    state_index: usize,
    /// More flexible state table
    state_table: T,
    /// More flexible machine state checker
    acceptance_checker: F
}

trait AcceptanceChecker{
    fn accept(&self, state: usize) -> bool;
}

trait StateStorage{
    fn lookup(&self, from: usize, symbol: char) -> Option<usize>;
    fn cache(&mut self, from: usize, to: usize, symbol: char);
}

impl<T: StateStorage, F: AcceptanceChecker> DFA<T, F>{
    /// Encode destination of dfa with symbol input
    fn encode(&mut self, s: char, from: usize, to: usize){
        self.state_table.cache(from, to, s);

        let table: [[char; 8];8] = {
            let a = 0;
            [['\0';8];8]
        };
    }

    /// Consume input symbol to forward state machine
    fn consume(&self, s: char) -> Option<usize> {
        self.state_table.lookup(self.state_index, s)
    }
}

pub struct BaseStorage{
    symbol_to_from: HashMap<char, usize>,
    from_to_to: HashMap<usize, usize>
}

pub struct BaseAcceptanceChecker{
    final_state: Vec<usize>
}

impl StateStorage for BaseStorage{
    fn lookup(&self, from: usize, symbol: char) -> Option<usize> {
        let cached_from = self.symbol_to_from[symbol]?;
        if from != cached_from{
            return None
        }

        self.from_to_to[from]
    }

    fn cache(&mut self, from: usize, to: usize, symbol: char) {
        self.symbol_to_from.insert(symbol, from);
        self.from_to_to.insert(from, to);
    }
}
impl <T: StateStorage, F: AcceptanceChecker> DFA<T,F>{
    pub fn transit(&mut self, symbol: char) -> Option<AutomataResponse>{
        if let Some(next_state) = self.state_table.lookup(self.state_index, symbol){
            return if self.acceptance_checker.accept(next_state) {
                Some(AutomataResponse::ACCEPT)
            } else {
                self.state_index = next_state;
                None
            }
        } else {
            Some(AutomataResponse::REJECT)
        }
    }

}

#[macro_export]
macro_rules! new_dfa {
    ($state_size:expr, $symbol_size:expr, [$($y:expr),*]) => {
                 /// Hacky things to make macro available to assign var in statement
                {
                    let base_acceptance_checker = BaseAcceptanceChecker{
                        final_state: vec![$($y),*]
                    };
                    let base_storage = BaseStorage{
                        symbol_to_from: HashMap::with_capacity($symbol_size),
                        from_to_to: HashMap::with_capacity($state_size)
                    };

                    DFA{
                        state_index: 0,
                        state_table: base_storage,
                        acceptance_checker: base_acceptance_checker
                    }
                }
    };
}
