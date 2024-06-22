
/// A dfa must be qualified to have abilities: Start, End, Reject, Accept, Move on string input
/// Each dfa node cell would be u32 data type, encoded to : 16-msb are dfa's option one, and 16-lsb are dfa's option two, each 8-msb are char, 8-lsb are index
pub struct DFA{
    index: u8,
    node_table: Vec<u32>
}

pub struct DFADef{

}

impl DFA{
    pub fn with_capacity(node_size: usize) -> DFA{
        DFA{
            index: 0,
            node_table: Vec::with_capacity(node_size)
        }
    }
}

