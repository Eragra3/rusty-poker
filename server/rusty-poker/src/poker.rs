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
    value: VoteValue,
    voter_uuid: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum VotingState {
    Closed,
    InProgress,
    Waiting,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Voting {
    start_datetime: DateTime<Utc>,
    votes: Vec<Vote>,
    title: String,
    state: VotingState,
    uuid: Uuid,
    host_uuid: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    uuid: Uuid,
    name: String,
    creation_datetime: DateTime<Utc>,
}

impl Vote {
    pub fn new(value: VoteValue, voter_uuid: Uuid) -> Vote {
        Vote {
            value: value,
            voter_uuid: voter_uuid,
        }
    }

    pub fn voter_uuid(&self) -> Uuid {
        self.voter_uuid
    }

    pub fn set_vote(&mut self, value: VoteValue) {
        self.value = value;
    }

    pub fn value(&self) -> VoteValue {
        self.value
    }
}

impl Voting {
    pub fn new(
        start_datetime: DateTime<Utc>,
        votes: Vec<Vote>,
        title: String,
        state: VotingState,
        uuid: Uuid,
        host_uuid: Uuid,
    ) -> Voting {
        Voting {
            start_datetime: start_datetime,
            votes: votes,
            title: title,
            state: state,
            uuid: uuid,
            host_uuid: host_uuid,
        }
    }

    pub fn votes(&self) -> &Vec<Vote> {
        &self.votes
    }

    pub fn votes_mut(&mut self) -> &mut Vec<Vote> {
        &mut self.votes
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn state(&self) -> VotingState {
        self.state
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
}

impl User {
    pub fn new(uuid: Uuid, name: String, creation_datetime: DateTime<Utc>) -> User {
        User {
            uuid: uuid,
            name: name,
            creation_datetime: creation_datetime,
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn creation_datetime(&self) -> DateTime<Utc> {
        self.creation_datetime
    }
}
