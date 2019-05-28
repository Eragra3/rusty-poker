use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VoteValue {
    Pending,
    Value(i32),
    Coffee,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vote {
    pub id: i32,
    pub value: VoteValue,
}

#[derive(Debug, Clone)]
pub struct Voting {
    pub id: i32,
    pub votes: Vec<Vote>,
}

impl Vote {
    pub fn new(id: i32, vote_value: VoteValue) -> Vote {
        Vote {
            id: id,
            value: vote_value,
        }
    }
}

impl Voting {
    pub fn new(id: i32, votes: Vec<Vote>) -> Voting {
        Voting {
            id: id,
            votes: votes,
        }
    }
}
