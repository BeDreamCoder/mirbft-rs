use std::collections::HashMap;

use crate::sequence::{Entry, Sequence, SequenceState};
use crate::*;

pub struct Bucket {
    pub leader: NodeID,
    pub id: BucketID,
    pub sequences: HashMap<SeqNo, Sequence>,
}

impl Bucket {
    pub fn new(leader: NodeID, id: BucketID) -> Self {
        Bucket {
            leader,
            id,
            sequences: HashMap::new(),
        }
    }

    pub fn apply_preprepare(&mut self, entry: Entry) {
        let mut sequence = Sequence::default();
        let seq_no = entry.seq_no;
        sequence.state = SequenceState::Preprepared;
        sequence.entry = entry;
        self.sequences.insert(seq_no, sequence);
    }
}
