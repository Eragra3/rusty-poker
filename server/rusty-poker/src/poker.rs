use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum VotingState {
    Closed,
    InProgress,
    Waiting
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voting {
    pub id: i32,
    pub start_datetime: DateTime<Utc>,
    pub votes: Vec<Vote>,
    pub title: String,
    pub state: VotingState,
    pub uuid: Uuid,
}

impl Vote {
    pub fn new(id: i32, value: VoteValue) -> Vote {
        Vote {
            id: id,
            value: value,
        }
    }

    pub fn set_vote(&mut self, value: VoteValue) {
        self.value = value;
    }
}

impl Voting {
    pub fn new(
        id: i32,
        start_datetime: DateTime<Utc>,
        votes: Vec<Vote>,
        title: String,
        state: VotingState,
        uuid: Uuid,
    ) -> Voting {
        Voting {
            id: id,
            start_datetime: start_datetime,
            votes: votes,
            title: title,
            state: state,
            uuid: uuid,
        }
    }
}
