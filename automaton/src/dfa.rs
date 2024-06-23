use std::collections::HashMap;

/// A dfa must be qualified to have abilities: Start, End, Reject, Accept, Move on string input
/// Each dfa node cell would be u32 data type, encoded to : 16-msb are dfa's option one, and 16-lsb are dfa's option two, each 8-msb are char, 8-lsb are index
pub struct DFA{
    state_index: u8,
    node_table: Vec<u32>
}

pub struct DFABuilder{
    start_node: DFANode,
    body: DFABodyBuilder
}

pub struct DFABodyBuilder{

}

pub trait DFAStartNode{
    fn new_start_node(state_name: String) -> impl DFABodyNode;
}

pub trait DFABodyNode{
    fn new(state_name: String) -> impl DFABodyNode;
}

pub trait DFAEndNode{}

pub struct DFANode{
    state_name: String,
    edges: HashMap<char, String>
}

impl DFABuilder{

}

impl DFAStartNode for DFANode{

    fn new_start_node(state_name: String) -> impl DFABodyNode{
        DFANode{
            state_name,
            edges: HashMap::with_capacity(0)
        }
    }

    pub fn new_end_node(state_name: String) -> DFANode{
        DFANode{
            state_name,
            edges: HashMap::with_capacity(0)
        }
    }

    pub fn transit_to(&mut self, input: char, state_name: String) -> &mut DFANode{
        self.edges.insert(input, state_name);
        &mut self
    }
}

impl DFABodyNode for DFANode{
    fn new(state_name: String) -> DFANode{
        DFANode{
            state_name,
            edges : HashMap::new()
        }
    }

}



impl DFA{
    pub fn with_capacity(node_size: usize) -> DFA{

    }
}

