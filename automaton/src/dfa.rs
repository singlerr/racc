use std::collections::HashMap;
use std::ops::Deref;
use regex::Regex;

/// A dfa must be qualified to have abilities: Start, End, Reject, Accept, Move on string input
/// Each dfa node cell would be u32 data type, encoded to : 16-msb are dfa's option one, and 16-lsb are dfa's option two, each 8-msb are char, 8-lsb are index
/// How to compile DFA to binary?
struct DFA<T: StateStorage>{
    /// Current state id of this machine.
    /// Initial value must be 0 because in tradition, initial state id is 0.
    state_index: u8,
    /// In order to define 2nd array in runtime, boxing it.
    /// Rows would be states and Columns symbols
    state_table: T
}


pub trait StateStorage{
    fn lookup(&self, from: u8, symbol: char) -> Option<u8>;
    fn cache(&mut self, from: u8, to: u8, symbol: char);
}

impl<T: StateStorage> DFA<T>{
    /// Encode destination of dfa with symbol input
    fn encode(&mut self, s: char, from: u8, to: u8){
        self.state_table.cache(from, to, s);
    }

    /// Consume input symbol to forward state machine
    fn consume(&self, s: char) -> Option<u8> {
        self.state_table.lookup(self.state_index, s)
    }
}
fn a(){

}

pub macro_rules! new_dfa {
    ($state_size:expr, $symbol_size:expr) => {

        struct GeneratedStorage_$state_size_$symbol_size{
            state_table: [[char; $symbol_size]; $state_size]
        };

        impl StateStorage for GeneratedStorage_$state_size_$symbol_size{
                fn lookup(&self, from: u8, symbol: char) -> Option<u8>{
                    if(from >= &self.state_table.len() || from < 0)
                        None()
                    Some(&self.state_table[from][symbol])
                }
                fn cache(&mut self, from: u8, to: u8, symbol: char){
                    &self.state_table[from][symbol] = to;
                }
        }

        DFA{
            state_index: 0,
            state_table: GeneratedStorage_$state_size_$symbol_size{
                state_table =
            }
        }
    };
}

fn main(){
    let dfa = new_dfa!(16,16);
}